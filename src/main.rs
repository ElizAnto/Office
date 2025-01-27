use rusqlite::{params, Connection, Result};
use druid::widget::{Button, Label, List, TextBox, Flex};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppState {
    note_input: String,
    task_input: String,
    notes: Arc<Vec<String>>,  // Список заметок
    tasks: Arc<Vec<String>>,  // Список задач
    budget_input: String,     // Ввод для суммы (доход или расход)
    income: f64,              // Доход
    expenses: f64,            // Расходы
    balance: f64,             // Баланс (доходы - расходы)
}

fn connect_db() -> Result<Connection> {
    let conn = Connection::open("my_notebook.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed INTEGER DEFAULT 0  -- добавляем поле completed с дефолтным значением
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS budget (
            id INTEGER PRIMARY KEY,
            income REAL,
            expenses REAL
        )",
        [],
    )?;
    Ok(conn)
}

fn load_notes(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT content FROM notes")?;
    let notes_iter = stmt.query_map([], |row| row.get(0))?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note?);
    }

    Ok(notes)
}

fn load_tasks(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT description FROM tasks")?;
    let tasks_iter = stmt.query_map([], |row| row.get(0))?;

    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

fn load_budget(conn: &Connection) -> Result<(f64, f64)> {
    let mut stmt = conn.prepare("SELECT income, expenses FROM budget LIMIT 1")?;
    let budget_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    for budget in budget_iter {
        return Ok(budget?);
    }

    // Если данных нет, возвращаем 0.0
    Ok((0.0, 0.0))
}

fn save_task(conn: &Connection, description: &str, completed: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        params![description, completed],
    )?;
    Ok(())
}

fn save_note(conn: &Connection, content: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (content) VALUES (?1)",
        params![content],
    )?;
    Ok(())
}

fn delete_note(conn: &Connection, content: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM notes WHERE content = ?1",
        params![content],
    )?;
    Ok(())
}

fn delete_task(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM tasks WHERE description = ?1",
        params![description],
    )?;
    Ok(())
}

fn build_ui() -> impl Widget<AppState> {
    let conn = connect_db().expect("Failed to connect to DB");
    let conn2 = connect_db().expect("Failed to connect to DB");

    let note_label = Label::new("Notes:");
    let task_label = Label::new("Tasks:");
    let budget_label = Label::new("Budget:");

    let note_input = TextBox::new()
        .with_placeholder("Enter a note...")
        .lens(AppState::note_input);

    let task_input = TextBox::new()
        .with_placeholder("Enter a task...")
        .lens(AppState::task_input);

    let budget_input = TextBox::new()
        .with_placeholder("Enter amount...")
        .lens(AppState::budget_input);

    let balance_label = Label::new(|data: &AppState, _env: &_| format!("Balance: {:.2}", data.balance));

    // Загрузим заметки и задачи из базы данных
    let notes_list = List::new(|| {
        Flex::row()
            .with_flex_child(Label::new(|item: &String, _env: &_| item.clone()).expand_width(), 1.0)
            .with_child(Button::new("Delete").on_click(|_ctx, item: &mut String, _env| {
                println!("Deleted task: {}", item);
            }))
    })
        .lens(AppState::notes);

    let tasks_list = List::new(|| {
        Flex::row()
            .with_flex_child(Label::new(|item: &String, _env: &_| item.clone()).expand_width(), 1.0)
            .with_child(Button::new("Delete").on_click(|_ctx, item: &mut String, _env| {
                println!("Deleted task: {}", item);
            }))
    })
        .lens(AppState::tasks);

    // Кнопка добавления заметки
    let note_button = Button::new("Add Note").on_click(move |_ctx, data: &mut AppState, _env| {
        if !data.note_input.trim().is_empty() {
            save_note(&conn2, &data.note_input).expect("Failed to save note"); // 0 - задача не выполнена
            let mut new_notes = Arc::make_mut(&mut data.notes);
            new_notes.push(data.note_input.clone());  // Добавляем новую заметку
            data.note_input.clear();  // Очищаем поле ввода
        }
    });

    // Кнопка добавления задачи
    let task_button = Button::new("Add Task").on_click(move |_ctx, data: &mut AppState, _env| {
        if !data.task_input.trim().is_empty() {
            save_task(&conn, &data.task_input, 0).expect("Failed to save task"); // 0 - задача не выполнена
            let mut new_tasks = Arc::make_mut(&mut data.tasks);
            new_tasks.push(data.task_input.clone());  // Добавляем новую задачу
            data.task_input.clear();  // Очищаем поле ввода
        }
    });


    // Кнопка добавления дохода
    let add_income_button = Button::new("Income").on_click(|_ctx, data: &mut AppState, _env| {
        if let Ok(income) = data.budget_input.trim().parse::<f64>() {
            data.income += income;
            data.balance = data.income - data.expenses;  // Пересчитываем баланс
            data.budget_input.clear();
        }
    });

    // Кнопка добавления расхода
    let add_expense_button = Button::new("Expense").on_click(|_ctx, data: &mut AppState, _env| {
        if let Ok(expense) = data.budget_input.trim().parse::<f64>() {
            data.expenses += expense;
            data.balance = data.income - data.expenses;  // Пересчитываем баланс
            data.budget_input.clear();
        }
    });

    // Строим интерфейс с использованием Flex для размещения элементов
    Flex::column()
        .with_child(note_label)
        .with_spacer(8.0)
        .with_child(notes_list)
        .with_spacer(8.0)
        .with_child(note_input)
        .with_spacer(8.0)
        .with_child(note_button)
        .with_spacer(16.0)
        .with_child(task_label)
        .with_spacer(8.0)
        .with_child(tasks_list)
        .with_spacer(8.0)
        .with_child(task_input)
        .with_spacer(8.0)
        .with_child(task_button)
        .with_spacer(16.0)
        .with_child(budget_label)
        .with_spacer(8.0)
        .with_child(balance_label)
        .with_spacer(8.0)
        .with_child(budget_input)
        .with_spacer(8.0)
        .with_child(add_income_button)
        .with_spacer(8.0)
        .with_child(add_expense_button)
}

fn main() {
    let conn = connect_db().expect("Failed to connect to DB");

    // Загрузка данных из базы данных
    let notes = load_notes(&conn).expect("Failed to load notes");
    let tasks = load_tasks(&conn).expect("Failed to load tasks");
    let (income, expenses) = load_budget(&conn).expect("Failed to load budget");

    let initial_state = AppState {
        note_input: String::new(),
        task_input: String::new(),
        budget_input: String::new(),
        notes: Arc::new(notes),
        tasks: Arc::new(tasks),
        income,
        expenses,
        balance: income - expenses,  // Расчитываем баланс из загруженных данных
    };

    let main_window = WindowDesc::new(build_ui())
        .title("My Notebook with Budget")
        .window_size((400.0, 800.0));

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}