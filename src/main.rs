use std::io::stdin;

fn main() {
    let mut entry_price = String::new();
    let mut stop_price = String::new();
    let mut target_price = String::new();
    let mut case = String::new();
    let mut risk = String::new();
    let mut leverage = String::new();
    let mut choose = String::new();
    println!("\n---Options---");
    println!("[1] Futures Case Management");
    println!("[2] Spot Case Management");
    stdin().read_line(&mut choose).expect("failed to read choose");
    let choose = choose.trim().parse().unwrap();
    match choose {
        1 => {
            println!("Enter the your entry price: ");
            stdin()
                .read_line(&mut entry_price)
                .expect("Failed to read entry price");

            println!("Enter the your stop price: ");
            stdin()
                .read_line(&mut stop_price)
                .expect("Failed to read stop price");

            println!("Enter the your target price: ");
            stdin()
                .read_line(&mut target_price)
                .expect("Failed to read target price");

            println!("Enter the your total balance: ");
            stdin().read_line(&mut case).expect("Failed to read case");

            println!("Enter the risk percentage: ");
            stdin().read_line(&mut risk).expect("Failed to read lvl");

            let entry_price: f32 = entry_price.trim().parse().expect("Invalid entry price");
            let stop_price: f32 = stop_price.trim().parse().expect("Invalid stop price");
            let target_price: f32 = target_price.trim().parse().expect("Invalid target price");
            let case: f32 = case.trim().parse().expect("Invalid case");
            let risk: i16 = risk.trim().parse().expect("Invalid lvl");

            if risk > 5 {
                println!("Warning! Risk is too high. Ideal risk is %1-%5");
            } else if risk < 0 || case < 0. || risk > 100 {
                panic!("Error! Risk or/and Case can't be lower than 0 or 100");
            }
            println!("Enter the leverage: ");
            stdin().read_line(&mut leverage).expect("Failed to read leverage");
            let leverage = leverage.trim().parse().expect("Invalid leverage");
            management_for_futures(entry_price, stop_price, target_price, risk, case, leverage)
        }
        2 => {
            println!("Enter the your entry price: ");
            stdin()
                .read_line(&mut entry_price)
                .expect("Failed to read entry price");

            println!("Enter the your stop price: ");
            stdin()
                .read_line(&mut stop_price)
                .expect("Failed to read stop price");

            println!("Enter the your target price: ");
            stdin()
                .read_line(&mut target_price)
                .expect("Failed to read target price");

            println!("Enter the your total balance: ");
            stdin().read_line(&mut case).expect("Failed to read case");

            println!("Enter the risk percentage: ");
            stdin().read_line(&mut risk).expect("Failed to read lvl");

            let entry_price: f32 = entry_price.trim().parse().expect("Invalid entry price");
            let stop_price: f32 = stop_price.trim().parse().expect("Invalid stop price");
            let target_price: f32 = target_price.trim().parse().expect("Invalid target price");
            let case: f32 = case.trim().parse().expect("Invalid case");
            let risk: i16 = risk.trim().parse().expect("Invalid lvl");

            if risk > 5 {
                println!("Warning! Risk is too high. Ideal risk is %1-%5");
            } else if risk < 0 || case < 0. || risk > 100 {
                panic!("Error! Risk or/and Case can't be lower than 0 or 100");
            }
            management_for_spot(entry_price, stop_price, target_price, risk, case)
        }
        _ => {
            println!("Invalid choice!");
        }
    }

    fn management_for_spot(entry_price: f32, stop_price: f32, target_price: f32, risk: i16, case: f32) {
        let money = (((entry_price / (entry_price - stop_price)) / 100.0) * risk as f32) * case;
        let lose = case * (risk as f32 / 100.);
        let profit = money * ((target_price - entry_price) / entry_price);
        let rr = profit / lose;
        let case = case + profit;
        if lose < 0. {
            panic!("Error");
        } else {
            println!("The amount you need to trade: {money}$");
            println!("Lose Amount: {lose}$");
            println!("Profit Amount: {profit}$");
            println!("R:R(Risk:Reward) rate: {rr}");
            println!("If you successful your case being {case}");
        }
    }
    fn management_for_futures(entry_price: f32, stop_price: f32, target_price: f32, risk: i16, case: f32, leverage: f32) {
        let money = (((entry_price / (entry_price - stop_price)) / (100.0 * leverage)) * risk as f32) * case;
        let lose = case * (risk as f32 / 100.);
        let profit = money * leverage * ((target_price - entry_price) / entry_price);
        let rr = profit / lose;
        let case = case + profit;
        if money < 0. {
            println!("Your position is Short");
            println!("The amount you need to trade: {:.2}$", money.abs());
            println!("Lose Amount: {lose}$");
            println!("Profit Amount: {profit}$");
            println!("R:R(Risk:Reward) rate: {rr}");
            println!("If you successful your case being {case}");
            if rr < 3. {
                println!("Warning! R:R rate is too low! {rr}")
            } else {
                println!("R:R(Risk:Reward) rate: {rr}");
            }
        } else {
            println!("Your position is Long.");
            println!("The amount you need to trade: {:.2}$", money);
            println!("Lose Amount: {lose}$");
            println!("Profit Amount: {profit}$");
            if rr < 3. {
                println!("Warning! R:R rate is too low! {rr}")
            }else {
                println!("R:R(Risk:Reward) rate: {rr}");
            }
        }
    }
}
