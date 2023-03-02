use std::thread::sleep;
use std::time::Duration;

fn main(){
    let x = 1000;

    sleep(Duration::from_millis(200));
    println!("У меня нет проблем");
    sleep(Duration::from_millis(300));
    println!("Кроме моей башки");
    sleep(Duration::from_millis(300));
    println!("Прости, я умер");
    sleep(Duration::from_millis(500));

    (0..x/7)
    .map(|n| x-n*7)
    .take_while(|&x| x>7)
    .for_each(|n| {
        println!("{} - 7 = {}", n, n-7);
        sleep(Duration::from_millis(500));
    });

    println!("Тепер ти мертвий всередині.");
}
