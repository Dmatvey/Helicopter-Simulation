//Dennis Matveyev
//Final Task
//Due : December 8th, 2021


fn main() {
    let helicopter = HelicopterData {
        fuel: 12,
        visibility: 80,
        index: 1
    };
    helicopter.api();
}

pub struct HelicopterData{
    fuel: i32,
    visibility: i32,
    index: i32
}

pub trait Flying {
    fn api(&self);
    fn enter_data() -> Vec<HelicopterData>;
    fn fly_helicopter(helicopter_list: &mut Vec<HelicopterData>);
}

impl Flying for HelicopterData {
    fn api(&self){
        let mut new_list:Vec<HelicopterData> = Self::enter_data();
        Self::fly_helicopter(&mut new_list);
    }

    fn enter_data() -> Vec<HelicopterData>{
        let mut list: Vec<HelicopterData> = Vec::new();
        let mut counter:u32 = 1;
        let mut num1 = String::new();
        println!("Enter number of Helicopters in the simulation:");
        std::io::stdin().read_line(&mut num1).expect("Unable to read entered data");
        let num_of_heli: i32 = num1.trim().parse().ok().expect("Program only processes numbers, enter a number");

        for i in 0..num_of_heli {
            let mut num2= String::new();
            println!("Enter the fuel level of helicopter {}:", counter);
            std::io::stdin().read_line(&mut num2).expect("Unable to read entered data");
            let fuel_level: i32 = num2.trim().parse().ok().expect("Program only processes numbers, enter a number");

            let mut num3 = String::new();
            println!("Enter the visibility for the helicopter {}:", counter);
            std::io::stdin().read_line(&mut num3).expect("Unable to read entered data");
            let vis: i32 = num3.trim().parse().ok().expect("Program only processes numbers, enter a number");

            let new_helicopter = HelicopterData {
                fuel: fuel_level,
                visibility: vis,
                index: i + 1
            };
            list.push(new_helicopter);
            counter = counter + 1;
        }
        list
    }

    fn fly_helicopter(helicopter_list: &mut Vec<HelicopterData>) {
        while helicopter_list.len() > 0{
            let mut i = 0;
            while i < helicopter_list.len(){
                if helicopter_list[i].visibility < 60{
                    println!("Not safe to fly");
                    helicopter_list.remove(i);
                }else if helicopter_list[i].fuel >= 10{
                    println!("Flying helicopter {}...now with fuel {}%", helicopter_list[i].index, helicopter_list[i].fuel);
                    helicopter_list[i].fuel = helicopter_list[i].fuel - 2;
                }else {
                    println!("Low fuel {}% landing helicopter {} now", helicopter_list[i].fuel, helicopter_list[i].index);
                    helicopter_list.remove(i);
                }
                i += 1;
            }
        }

    }
}