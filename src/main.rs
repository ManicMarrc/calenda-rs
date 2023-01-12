fn get_month_day_count(month: i32, year: i32) -> i32 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    2 if year % 4 == 0 => 29,
    2 | 4 | 6 | 9 | 11 => 28,
    _ => unreachable!(),
  }
}

fn get_day_name(day: i32, month: i32, year: i32) -> String {
  chrono::NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap().format("%A").to_string()
}

fn main() {
  let today = chrono::Local::now();
  let date = today.format("%d %m %Y").to_string();
  let mut date = date.split(' ');

  let day = date.next().unwrap().parse::<i32>().unwrap();
  let month = date.next().unwrap().parse::<i32>().unwrap();
  let year = date.next().unwrap().parse::<i32>().unwrap();

  println!("{:^27}", today.format("%B %d, %Y"));
  println!("Sun Mon Tue Wed Thu Fri Sat");
  match get_day_name(1, month, year).as_str() {
    "Sunday" => (),
    "Monday" => print!("   "),
    "Tuesday" => print!("      "),
    "Wednesday" => print!("         "),
    "Thursday" => print!("            "),
    "Friday" => print!("               "),
    "Saturday" => print!("                  "),
    _ => (),
  }

  for i in 1..=get_month_day_count(month, year) {
    if i == day {
      print!("\x1B[47m{i}\x1B[0m ");
    } else {
      print!("{i} ");
      if i < 10 {
        print!(" ");
      }
    }

    if get_day_name(i, month, year) == "Saturday" {
      println!();
    } else {
      print!(" ");
    }
  }
}
