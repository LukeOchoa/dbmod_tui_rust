use dbmod_tui3::db_tools::execute_rebuild;
use dbmod_tui3::make_tables_and_rows;
use std::collections::HashMap;
use std::error::Error;

type MagicError = Box<dyn Error>;

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn quick_maker(amount: i32, character: &str) -> String {
    let mut s = String::default();
    for _ in 0..amount {
        s = format!("{}{}", s, character)
    }
    s
}

fn newliner(amount: i32) -> String {
    quick_maker(amount, "\n")
}

fn tabber(amount: i32) -> String {
    quick_maker(amount, "\t")
}

fn get_input() -> Result<String, String> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let error = stdin.read_line(&mut buffer);
    if let Err(err) = error {
        return Err(err.to_string());
    }
    buffer.pop();
    Ok(buffer)
}

fn validate_input<T>(input: &str, acceptable_input: &Vec<T>) -> Result<usize, String> {
    //! Check if input is something... dont know lol
    //!
    //! losing marbles
    let input = match input.parse::<usize>() {
        Ok(input) => input,
        Err(err) => {
            return Err(format!("Err: {}", err));
        }
    };

    if input < acceptable_input.len() {
        Ok(input)
    } else {
        Err("Err: Input is not valid".to_string())
    }
}

struct Screen {
    s: String,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            s: String::default(),
        }
    }

    fn _concat(&mut self, s: &str) {
        self.s = format!("{}{}", self.s, s)
    }
    fn concat_with_newlines(&mut self, s: &str, before_amount: i32, after_amount: i32) {
        self.s = format!(
            "{}{}{}{}",
            self.s,
            newliner(before_amount),
            s,
            newliner(after_amount)
        )
    }

    fn print_screen(&self) {
        clear_terminal();
        println!("{}", self.s);
    }
    fn print_and_clean_screen(&mut self) {
        clear_terminal();
        println!("{}", self.s);
        self.clear();
    }
    fn clear(&mut self) {
        self.s = String::default();
    }
    //fn _print_err(&mut self, err: &str) {
    //    self.concat(err);
    //    self.print_screen();
    //    self.clear();
    //}
}

// Vec<Statelet>
struct AppState {
    state: HashMap<String, Package>,
}

struct Special {
    list: Vec<String>,
    operator: fn(&Vec<String>) -> Result<(), String>,
    chosen: String,
}
impl Special {
    fn new(
        list: Vec<String>,
        operator: fn(&Vec<String>) -> Result<(), String>,
        chosen: String,
    ) -> Special {
        Special {
            list,
            operator,
            chosen,
        }
    }
}
enum Property {
    Action(bool),
    Selectable,
    Togglable(bool),
    List(Special),
}
//if let Property::Togglable(booly) = self {
//    if *booly {
//        format!("{}X", tabber(1))
//    } else {
//        String::default()
//    }
//} else {
//    String::default()
//}
impl Property {
    fn is_toggled_bool(&self) -> bool {
        if let Property::Togglable(booly) = self {
            return *booly;
        }
        false
    }
    fn is_toggled(&self) -> String {
        if let Property::Togglable(booly) = self {
            if *booly {
                //println!("Returning an X with a tab before it...");
                //get_input();
                return format!("{}X", tabber(1));
            }
        }

        String::default()
    }
    fn is_selectable(&self) -> Option<bool> {
        if let Property::Action(booly) = self {
            return Some(*booly);
        }
        None
    }
}

struct Package {
    statelets: Vec<Statelet>,
    viewer_updater: fn(Screen, &Vec<Statelet>) -> Screen,
}
impl Package {
    fn new(
        statelets: Vec<Statelet>,
        viewer_updater: fn(Screen, &Vec<Statelet>) -> Screen,
    ) -> Package {
        Package {
            statelets,
            viewer_updater,
        }
    }
}

struct Statelet {
    name: String,
    property: Property,
}
impl Statelet {
    fn new(name: &str, property: Property) -> Statelet {
        Statelet {
            name: name.to_string(),
            property,
        }
    }
}
//fn basic_screen_viewer(options: &Vec<Statelet>) {
//    let mut screen = Screen::new();
//    for (index, element) in options.iter().enumerate() {
//        let element = &format!("{}) -> {}{}", index, element.name, newliner(1));
//        screen.concat(element);
//    }
//
//    screen.print_screen();
//    screen.clear();
//}

fn homepage_viewer_updater(mut screen: Screen, list_of_statelets: &Vec<Statelet>) -> Screen {
    for (index, statelet) in list_of_statelets.iter().enumerate() {
        if let Some(booly) = statelet.property.is_selectable() {
            if booly {}
        }
        let line = format!(
            "{}) -> {}{}",
            index,
            statelet.name,
            statelet.property.is_toggled()
        );
        screen.concat_with_newlines(&line, 1, 0);
    }

    screen
}

fn rebuild_viewer_updater(mut screen: Screen, list_of_statelets: &Vec<Statelet>) -> Screen {
    if let Property::Action(booly) = list_of_statelets.get(2).unwrap().property {
        if let Property::List(special) = &list_of_statelets.get(0).unwrap().property {
            if booly {
                let (tables, rows) = test_load_files().unwrap();
                let (tables, rows) = make_tables_and_rows(tables, rows).unwrap();
                let rows = if list_of_statelets.get(1).unwrap().property.is_toggled_bool() {
                    Some(rows)
                } else {
                    None
                };
                execute_rebuild(tables, rows, &special.chosen).unwrap();
                let success = &format!(
                    "Table: |{}| has been successfully updated...!",
                    special.chosen
                );
                screen.concat_with_newlines(success, 2, 2);
                return screen;
            }
        }
    }

    if let Property::List(special) = &list_of_statelets.get(0).unwrap().property {
        screen.concat_with_newlines(&format!("Chosen file: |{}|", special.chosen), 0, 2);
    }
    for (index, element) in list_of_statelets.iter().enumerate() {
        let line = format!(
            "{}) -> {}{}",
            index,
            element.name,
            element.property.is_toggled()
        );
        screen.concat_with_newlines(&line, 1, 0);
    }

    screen
}

fn completely_validate_input<T>(
    list: &Vec<T>,
    input: Result<String, String>,
) -> Result<usize, String> {
    let input = input?;
    if input == "\t" {
        // if user wants to go back to previous item
        return Err(input);
    }
    let input = validate_input(&input, list)?;
    Ok(input)
}

fn basic_immediate(
    mut list_of_statelets: Vec<Statelet>,
    viewer_updater: fn(Screen, &Vec<Statelet>) -> Screen,
) -> (String, Package) {
    let mut screen = Screen::new();

    //basic_screen_viewer(&list_of_statelets);
    viewer_updater(Screen::new(), &list_of_statelets).print_screen();

    loop {
        match completely_validate_input(&list_of_statelets, get_input()) {
            Ok(input) => {
                let statelet = list_of_statelets.get_mut(input).unwrap();
                let payload: Option<String> = match &mut statelet.property {
                    Property::Togglable(booly) => {
                        statelet.property = Property::Togglable(!*booly);
                        None
                    }
                    Property::Selectable => {
                        return (
                            statelet.name.clone(),
                            Package::new(list_of_statelets, viewer_updater),
                        )
                    }
                    Property::Action(booly) => {
                        statelet.property = Property::Action(!*booly);
                        Some(statelet.name.clone())
                    }
                    Property::List(special) => {
                        let mut cnt = 0;
                        special.list.iter().for_each(|element| {
                            screen.concat_with_newlines(&format!("{}): |{}|", cnt, element), 0, 1);
                            cnt = cnt + 1;
                        });
                        screen.concat_with_newlines(
                            &format!("The Chosen File is: -> |{}|", special.chosen),
                            1,
                            1,
                        );
                        screen.print_and_clean_screen();
                        match completely_validate_input(&special.list, get_input()) {
                            Ok(input) => {
                                special.chosen = special.list.get(input).unwrap().clone();
                            }
                            Err(err) => {
                                if err != "\t" {
                                    screen.concat_with_newlines(&err, 0, 2);
                                }
                            }
                        }
                        None
                    }
                };
                if let Some(_) = payload {
                    viewer_updater(Screen::new(), &list_of_statelets).print_screen();
                    _ = get_input();
                    list_of_statelets.iter_mut().for_each(|statelet| {
                        if let Property::Action(_) = statelet.property {
                            statelet.property = Property::Action(false);
                        }
                    });
                }
            }
            Err(err) => {
                if err == "\t" {
                    return (err, Package::new(list_of_statelets, viewer_updater));
                }
                screen.concat_with_newlines(&err, 0, 2);
            }
        }
        screen = viewer_updater(screen, &list_of_statelets);
        screen.print_and_clean_screen();
    }
}

fn vec_str_to_vec_string(vec: Vec<&str>) -> Vec<String> {
    let mut vec_string = Vec::new();
    vec.iter().for_each(|elem| {
        vec_string.push(elem.to_string());
    });

    vec_string
}

fn get_files(_list: &Vec<String>) -> Result<(), String> {
    Ok(())
}

fn set_homepage() -> Package {
    let statelets = vec![
        Statelet::new("rebuild", Property::Selectable),
        Statelet::new("view", Property::Togglable(false)),
    ];
    Package {
        statelets,
        viewer_updater: homepage_viewer_updater,
    }
}
fn set_rebuild() -> Package {
    // List Of Files
    let files = vec_str_to_vec_string(vec!["one", "two", "three", "four", "five"]);

    // Rebuild
    let statelets = vec![
        Statelet::new(
            "list of files",
            Property::List(Special::new(files, get_files, String::default())),
        ),
        Statelet::new("insert rows in table", Property::Togglable(false)),
        Statelet::new("are you sure you want to rebuild?", Property::Action(false)),
    ];

    Package {
        statelets,
        viewer_updater: rebuild_viewer_updater,
        //viewer_updater: homepage_viewer_updater,
        //viewer_updater: rebuild_viewer_updater,   // Fix This...
    }
}
fn set_app_state() -> AppState {
    let mut app_state = AppState {
        state: HashMap::new(),
    };

    // Homepage
    app_state
        .state
        .insert(String::from("homepage"), set_homepage());

    // Rebuild
    app_state
        .state
        .insert(String::from("rebuild"), set_rebuild());

    app_state
}

fn reloadables(mut app_state: AppState) -> AppState {
    // reload filenames in rebuild/list_of_files/special/list

    let (tables, rows) = test_load_files().unwrap();
    let (tables, _rows) = make_tables_and_rows(tables, rows).unwrap();
    let mut tables_row = Vec::new();
    tables
        .iter()
        .for_each(|(key, _)| tables_row.push(key.to_string()));

    let prop = &mut app_state
        .state
        .get_mut("rebuild")
        .unwrap()
        .statelets
        .get_mut(0)
        .unwrap()
        .property;

    if let Property::List(_) = prop {
        *prop = Property::List(Special {
            list: tables_row,
            operator: get_files,
            chosen: String::default(),
        });
    }

    app_state
}

fn set_history() -> HashMap<String, String> {
    let mut history = HashMap::new();
    history.insert("rebuild".to_string(), "homepage".to_string());
    history.insert("view".to_string(), "homepage".to_string());

    history.insert("list of files".to_string(), "rebuild".to_string());
    history.insert("insert rows in table".to_string(), "rebuild".to_string());
    history.insert(
        "are you sure you want to rebuild?".to_string(),
        "rebuild".to_string(),
    );
    history.insert("homepage".to_string(), "homepage".to_string());

    history
}
use dbmod_tui3::test_load_files;

fn testing_things() -> Result<(), MagicError> {
    let (tables, rows) = test_load_files()?;
    let (tables, rows) = make_tables_and_rows(tables, rows)?;

    tables
        .iter()
        .for_each(|(key, element)| println!("{}:{}", key, element));
    print!("{}", newliner(3));

    rows.iter().for_each(|(key, element)| {
        println!("key:{}", key);
        element
            .iter()
            .for_each(|element| println!("-> {}{}", tabber(1), element));
        print!("{}", newliner(1))
    });

    Ok(())
}

fn main() {
    if let Err(err) = testing_things() {
        panic!("{}", err)
    } else {
        _ = get_input();
    }
    let mut vibe: String;
    let mut package: Package;
    let mut previous: String;

    let mut app_state = set_app_state();
    let history = set_history();

    // Prime Initial Run \\

    // Move package for use
    package = app_state.state.remove("homepage").unwrap();
    // Set chosen key
    previous = "homepage".to_string();
    // give package to immediate handler
    (vibe, package) = basic_immediate(package.statelets, package.viewer_updater);
    // return package to its original position
    app_state.state.insert(previous.clone(), package);

    // handle bad input/reversing
    if &vibe == "\t" {
        vibe = history.get(&previous).unwrap().clone()
    }

    // Magic // Main Loop // crash the program to end it lol
    loop {
        // *Move* a (Package) out of the app state
        println!("vibe: |{}|", vibe);
        package = app_state.state.remove(&vibe).unwrap();

        // Give it to the immediate mode function
        // Then return the same (Package) after it has been modified
        let og_vibe = vibe.clone();
        (vibe, package) = basic_immediate(package.statelets, package.viewer_updater);

        // Handle History
        if &vibe == "\t" {
            vibe = history.get(&previous).unwrap().clone()
        }
        previous = vibe.clone();

        // *Move* the value back into the app state
        app_state.state.insert(og_vibe.clone(), package);

        // Reload things that need to be updated lol
        app_state = reloadables(app_state);
    }
}

//println!("statelet chosen");
//package
//    .statelets
//    .iter()
//    .for_each(|element| println!("{}", element.name));
//println!("vibe chosen: {}", vibe);
//get_input();

// fn do_this(
//     viewer: impl Fn(&Vec<&str>) -> Screen,
//     updater: impl Fn(&str, &Vec<&str>) -> Result<usize, String>,
//     options: &Vec<&str>,
// ) -> usize {
//     let mut screen = Screen::new();
//
//     viewer(options).print_screen();
//
//     loop {
//         let input = get_input();
//         match input {
//             Ok(input) => match updater(&input, options) {
//                 Ok(index) => return index,
//                 Err(err) => screen.concat(&err),
//             },
//             Err(err) => screen.concat(&err),
//         };
//         screen.concat_with_newlines(&viewer(options).s, 3, 0);
//         screen.print_screen();
//         screen.clear();
//     }
// }
//
// enum Selectable {
//     Yes(String),
//     No(String),
// }
// fn do_all(
//     viewer: impl Fn(&Vec<Selectable>) -> Screen,
//     updater: impl Fn(&str, &Vec<Selectable>) -> Result<usize, String>,
//     options: &Vec<Selectable>,
// ) -> usize {
//     let mut screen = Screen::new();
//
//     viewer(options).print_screen();
//
//     loop {
//         let input = get_input();
//     }
// }
// //fn do_all() {
// //
// //}
//
// // CUSTOMS \\
//
// fn basic_viewer(options: &Vec<&str>) -> Screen {
//     let mut screen = Screen::new();
//     for (index, element) in options.iter().enumerate() {
//         let element = &format!("{}) -> {}{}", index, element, newliner(1));
//         screen.concat(element);
//     }
//
//     screen
// }
//
// // Homepage
// fn homepage_viewer(options: &Vec<&str>) -> Screen {
//     basic_viewer(options)
// }
//
// fn homepage_updater(input: &str, options: &Vec<&str>) -> Result<usize, String> {
//     validate_input(input, options)
// }
//
// // Rebuild
// fn rebuild_viewer(options: &Vec<&str>, input: Option<usize>) -> Screen {
//     let mut screen = Screen::new();
//     for (index, element) in options.iter().enumerate() {
//         let selected = match input {
//             Some(usizey) => {
//                 if usizey == index {
//                     format!("{}X", tabber(1))
//                 } else {
//                     "".to_string()
//                 }
//             }
//             None => "".to_string(),
//         };
//         let element = &format!("{}) -> {}{}{}", index, element, selected, newliner(1));
//         screen.concat(element);
//     }
//
//     screen
// }
// fn rebuild_updater(input: &str, options: &Vec<&str>) -> Result<usize, String> {
//     validate_input(input, options)
// }
