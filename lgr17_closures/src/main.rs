use std::thread;
use std::time::Duration;

/* 17.0 closures fitness fn */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

// 1. instead of u32 could be used generic
// 2. instead of holding a value one could replace it to hashMap
struct Cacher<T>
    where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // 1. if we set this function it will run event if it's not necessary (random_number ==3)
    // that is why we use closures 'expensive_closure' which instead holding value hold fn itself

    // 2.closures will deduct type after first function and set types as basic (-later different types could cause an error)
    /* start example  */
    let example_closure = |x| x;

    let s = example_closure(String::from("siemanko!"));
    // let n  = example_closure(55); //expecting error
    /* end example */

    let mut cache_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache_result.value(intensity));
        }
    }
}
