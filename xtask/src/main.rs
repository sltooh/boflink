use tasks::TASKLIST;

mod tasks;
mod utils;

#[cfg(feature = "dist")]
mod dist;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() == 1 {
        tasks::print_help()?;
        return Ok(());
    }

    #[cfg(feature = "dist")]
    if std::env::args().nth(1).is_some_and(|task| task == "dist") {
        tasks::dist()?;
        return Ok(());
    }

    for task in std::env::args().skip(1) {
        if !TASKLIST.iter().any(|defined| defined.name == task) {
            return Err(format!("unknown task '{task}'").into());
        }
    }

    for task in std::env::args().skip(1) {
        if let Some(defined) = TASKLIST.iter().find(|defined| defined.name == task) {
            (defined.run)()?;
        } else {
            return Err(format!("unknown task '{task}'").into());
        }
    }

    Ok(())
}
