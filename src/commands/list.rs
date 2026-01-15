use crate::config::Config;
use crate::error::Result;

pub fn list(tag: Option<String>, recent: bool) -> Result<()> {
    let config = Config::load()?;

    if config.marks.is_empty() {
        println!("No marks yet. Add one with the add command <name>");
        return Ok(());
    }

    let mut marks: Vec<_> = config
        .marks
        .iter()
        .filter(|(_, mark)| {
            if let Some(ref filter_tag) = tag {
                mark.tags.contains(filter_tag)
            } else {
                true
            }
        })
        .collect();

    if recent {
        marks.sort_by(|a, b| b.1.last_accessed.cmp(&a.1.last_accessed));
    } else {
        marks.sort_by_key(|(_, mark)| mark.name.as_str());
    }

    for (path, mark) in marks {
        let name = &mark.name;
        let tags_str = if mark.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", mark.tags.join(", "))
        };

        println!("{}{}", name, tags_str);
        println!("  → {}", path.display());

        if recent {
            let duration = chrono::Utc::now()
                .signed_duration_since(mark.last_accessed)
                .num_days();
            if duration == 0 {
                println!("  (accessed today)");
            } else if duration == 1 {
                println!("  (accessed yesterday)");
            } else {
                println!("  (accessed {} days ago)", duration);
            }
        }
        println!();
    }

    Ok(())
}
