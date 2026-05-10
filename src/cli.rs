use clap::{Parser, Subcommand};
use crate::app::state::{Appstate, ItemType, TodoItem};

const G: &str = "\x1b[32m";
const C: &str = "\x1b[36m";
const D: &str = "\x1b[2m";
const B: &str = "\x1b[1m";
const R: &str = "\x1b[0m";

#[derive(Parser)]
#[command(name = "rudo", about = "Terminal todo manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "List tasks in the active list")]
    List,
    #[command(about = "Show all lists")]
    Lists,
    #[command(about = "Add a task  (no quotes needed)")]
    Add {
        #[arg(num_args = 1..)]
        text: Vec<String>,
    },
    #[command(about = "Mark task N as done")]
    Done { n: usize },
    #[command(about = "Uncheck task N")]
    Undo { n: usize },
    #[command(about = "Remove task N")]
    Rm { n: usize },
    #[command(about = "Switch the active list by name")]
    Use { name: String },
    #[command(about = "Show completion stats")]
    Status,
}

pub fn run_cli(cmd: Commands) {
    let mut state = match crate::io::load_state() {
        Some(mut s) => { s.theme = s.get_theme(); s }
        None => Appstate::default(),
    };

    let mutates = matches!(cmd, Commands::Add { .. } | Commands::Done { .. } | Commands::Undo { .. } | Commands::Rm { .. } | Commands::Use { .. });

    match cmd {
        Commands::List      => print_list(&state),
        Commands::Lists     => print_lists(&state),
        Commands::Status    => print_status(&state),
        Commands::Add { text } => add_task(&mut state, text.join(" ")),
        Commands::Done { n } => set_done(&mut state, n, true),
        Commands::Undo { n } => set_done(&mut state, n, false),
        Commands::Rm { n }   => remove_task(&mut state, n),
        Commands::Use { name } => switch_list(&mut state, name),
    }

    if mutates {
        let _ = crate::io::save_state(&state);
    }
}

fn task_indices(state: &Appstate) -> Vec<usize> {
    state.lists.get(state.current_list_index)
        .map(|l| l.items.iter().enumerate()
            .filter(|(_, i)| i.item_type == ItemType::Task)
            .map(|(idx, _)| idx)
            .collect())
        .unwrap_or_default()
}

fn print_list(state: &Appstate) {
    let list = match state.lists.get(state.current_list_index) {
        Some(l) => l,
        None => { eprintln!("No lists found."); return; }
    };
    let tasks: Vec<_> = list.items.iter().filter(|i| i.item_type == ItemType::Task).collect();
    let done = tasks.iter().filter(|i| i.is_done).count();
    println!();
    println!(" {B}{}{R}  {D}[{done}/{}]{R}", list.name, tasks.len());
    println!(" {D}──────────────────────────{R}");
    if list.items.is_empty() {
        println!("  {D}No tasks. Use 'rudo add <task>' to add one.{R}");
    } else {
        let mut n = 1usize;
        for item in &list.items {
            match item.item_type {
                ItemType::Separator => println!("  {D}──────────────────────{R}"),
                ItemType::Header    => println!("  {B}{}{R}", item.description.to_uppercase()),
                ItemType::Task => {
                    let (cb, dim) = if item.is_done {
                        (format!("{G}[✓]{R}"), format!("{D}"))
                    } else {
                        (format!("{D}[ ]{R}"), String::new())
                    };
                    let sub = if !item.sub_items.is_empty() {
                        let sd = item.sub_items.iter().filter(|s| s.is_done).count();
                        format!("  {D}({sd}/{}){R}", item.sub_items.len())
                    } else { String::new() };
                    println!("  {D}{n:2}.{R} {cb} {dim}{}{R}{sub}", item.description);
                    n += 1;
                }
            }
        }
    }
    println!();
}

fn print_lists(state: &Appstate) {
    println!();
    for (i, list) in state.lists.iter().enumerate() {
        let tasks: Vec<_> = list.items.iter().filter(|i| i.item_type == ItemType::Task).collect();
        let done = tasks.iter().filter(|i| i.is_done).count();
        let badge = if tasks.is_empty() { String::new() } else { format!("[{done}/{}]", tasks.len()) };
        if i == state.current_list_index {
            println!("  {C}→ {B}{:<24}{R} {D}{badge}{R}", list.name);
        } else {
            println!("    {:<24} {D}{badge}{R}", list.name);
        }
    }
    println!();
}

fn add_task(state: &mut Appstate, desc: String) {
    if desc.trim().is_empty() {
        eprintln!("Task description cannot be empty.");
        return;
    }
    let list_name = state.lists.get(state.current_list_index).map(|l| l.name.clone()).unwrap_or_default();
    if let Some(list) = state.lists.get_mut(state.current_list_index) {
        list.items.push(TodoItem {
            is_done: false,
            description: desc.clone(),
            sub_items: vec![],
            item_type: ItemType::Task,
        });
        println!("  {G}+{R} Added {B}\"{desc}\"{R} to {C}{list_name}{R}");
    } else {
        eprintln!("No active list.");
    }
}

fn set_done(state: &mut Appstate, n: usize, done: bool) {
    let indices = task_indices(state);
    let real = match indices.get(n.saturating_sub(1)) {
        Some(&i) => i,
        None => { eprintln!("No task #{n}."); return; }
    };
    let list = state.lists.get_mut(state.current_list_index).unwrap();
    if let Some(item) = list.items.get_mut(real) {
        item.is_done = done;
        for sub in item.sub_items.iter_mut() { sub.is_done = done; }
        if done {
            println!("  {G}✓{R} Done: {B}\"{}\"{R}", item.description);
        } else {
            println!("  ○ Unchecked: {B}\"{}\"{R}", item.description);
        }
    }
}

fn remove_task(state: &mut Appstate, n: usize) {
    let indices = task_indices(state);
    let real = match indices.get(n.saturating_sub(1)) {
        Some(&i) => i,
        None => { eprintln!("No task #{n}."); return; }
    };
    let list = state.lists.get_mut(state.current_list_index).unwrap();
    let desc = list.items[real].description.clone();
    list.items.remove(real);
    println!("  {D}Removed:{R} {B}\"{desc}\"{R}");
}

fn switch_list(state: &mut Appstate, name: String) {
    let idx = state.lists.iter().position(|l| l.name.to_lowercase().contains(&name.to_lowercase()));
    match idx {
        Some(i) => {
            state.current_list_index = i;
            println!("  {C}→{R} Switched to {B}{}{R}", state.lists[i].name);
        }
        None => eprintln!("  No list matching \"{name}\"."),
    }
}

fn print_status(state: &Appstate) {
    let (mut ti, mut di, mut ts, mut ds) = (0usize, 0, 0, 0);
    for list in &state.lists {
        for item in &list.items {
            if item.item_type == ItemType::Task {
                ti += 1; if item.is_done { di += 1; }
            }
            for sub in &item.sub_items { ts += 1; if sub.is_done { ds += 1; } }
        }
    }
    let total = ti + ts;
    let done = di + ds;

    let bar = |n: usize, d: usize| -> String {
        let fill = if n > 0 { (d as f32 / n as f32 * 14.0).round() as usize } else { 0 };
        format!("{G}{}{R}{D}{}{R}", "▰".repeat(fill), "▱".repeat(14usize.saturating_sub(fill)))
    };
    let pct = |n: usize, d: usize| if n > 0 { d * 100 / n } else { 0 };

    println!();
    println!("  {D}Lists  {R} {B}{}{R}", state.lists.len());
    println!("  {D}Items  {R} {}  {di}/{ti}  {B}{}%{R}", bar(ti, di), pct(ti, di));
    println!("  {D}Sub    {R} {}  {ds}/{ts}  {B}{}%{R}", bar(ts, ds), pct(ts, ds));
    println!("  {D}Total  {R} {}  {done}/{total}  {B}{}%{R}", bar(total, done), pct(total, done));
    println!();
}
