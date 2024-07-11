use console;
use rand::{self, Rng};
use std::collections::HashMap;
fn main() {
    start();
}

// 分类浏览商品
// 根据名称查询商品
// 下单， 如果库存不足，则显示下单失败
// 支付订单，如果余额不足，则显示支付失败
// 统计自己订单的总金额

#[derive(Debug, Clone)]
struct Category(String, String);

#[derive(Debug)]
struct Product {
    name: String,
    category: Category,
    price: f32,
    stock: u32,
    id: u32,
}

fn handle_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("输入错误");
    input
}
fn get_product_names(products: &Vec<Product>) -> Vec<&String> {
    let res: Vec<&String> = products.iter().map(|p| &p.name).collect();
    res
}
fn render_categories(
    products: &Vec<Product>,
    categories: &HashMap<&str, &str>,
    mut update_step: impl FnMut(i32),
) {
    println!("{}", console::style("商品分类如下：").bold().blue());
    println!("{}", console::style("=".repeat(32)).bold().blue());

    let mut selections: Vec<(usize, &str)> = Vec::new();
    for (index, (_key, desc)) in categories.iter().enumerate() {
        println!("[{}]: {}", index + 1, desc);
        selections.push((index, desc));
    }
    render_go_home(categories.len() as i32 + 1);
    println!("{}", console::style("=".repeat(32)).bold().blue());

    println!("{}", console::style("请输入分类编号").blue().bold());

    let input = handle_input();
    let selection: i32 = input.trim().parse().expect("输入错误");
    let category: Vec<&str> = selections
        .iter()
        .filter(|i| i.0 as i32 == selection - 1)
        .map(|s| s.1)
        .collect();
    if category.len() > 0 {
        let items = get_product_by_category(&products, category[0]);
        show_products(&items);
        std::thread::sleep(std::time::Duration::from_millis(800));
        render_categories(&products, &categories, update_step);
    } else {
        if selection == categories.len() as i32 + 1 {
            update_step(100);
            return;
        }
        println!(
            "{}",
            console::style("您输入的选项无效, 请重新输入").red().bold()
        );
        render_categories(products, categories, update_step);
    }
}
fn render_home() {
    let welcome_text = "=======欢迎使用商品管理系统=====";
    let welcome_text_count = welcome_text.chars().count() + 10;
    println!("{}", console::style(welcome_text).bold().blue());

    println!("[1]: 分类浏览商品");
    println!("[2]: 根据名称查询商品");
    println!("[3]: 下单 & 支付");
    println!("[4]: 充值");
    println!("[5]: 查看余额");
    println!("[6]: 退出");
    println!("[7]: 添加购物车");

    println!(
        "{}",
        console::Style::new()
            .blue()
            .bold()
            .apply_to("=".repeat(welcome_text_count))
    );
    println!(
        "{}",
        console::Style::new()
            .blue()
            .bold()
            .apply_to("请输入操作编号：")
    );
}

fn render_all_products(products: &Vec<Product>) -> (HashMap<usize, &str>, Vec<&String>) {
    let mut names = get_product_names(&products);
    let mut count = 0;
    let max_len = names.iter().map(|s| s.len()).max().unwrap();
    let mut map: HashMap<usize, &str> = HashMap::new();
    for (index, name) in names.iter_mut().enumerate() {
        let mut new_name = name.clone();
        map.insert(index, name);
        new_name.push_str(" ".repeat(max_len - name.len()).as_str());
        print!("[{:>2}]: {}\t", index + 1, new_name);
        count += 1;
        if count % 4 == 0 {
            println!("\n");
        }
    }
    render_go_home(names.len() as i32 + 1);
    (map, names)
}
fn render_product_by_name(products: &Vec<Product>, mut update_step: impl FnMut(i32)) {
    println!("{}", console::style("=".repeat(100)).blue().bold());
    let (map, names) = render_all_products(products);
    println!("{}", console::style("=".repeat(100)).blue().bold());
    println!(
        "{}{}{}",
        console::style("=".repeat(30)).blue().bold(),
        console::style("请输入要查询的商品编号: ").blue().bold(),
        console::style("=".repeat(30)).blue().bold(),
    );
    let input = handle_input();
    let selection: i32 = input.trim().parse().expect("输入错误");
    if selection == (names.len() + 1) as i32 {
        update_step(100);
        return;
    }
    let key = selection - 1;
    if let Some(name) = map.get(&(key as usize)) {
        let item = get_product_by_name(&products, name).unwrap();
        println!(
            "{}{}{}",
            console::style("=".repeat(30)).blue().bold(),
            console::style("您要查询的商品如下:").blue().bold(),
            console::style("=".repeat(30)).blue().bold(),
        );
        show_products(&vec![item]);
        std::thread::sleep(std::time::Duration::from_millis(800));
    } else {
        println!(
            "{}",
            console::Style::new()
                .bg(console::Color::Red)
                .white()
                .bold()
                .apply_to("您输入的选项无效")
        );
    }
}
fn pay(
    orders: Vec<Order>,
    mut update_balance: impl FnMut(f32) -> f32,
    mut update_step: impl FnMut(i32),
) {
    // TODO: 待添加校验 库存是否充足逻辑
    if orders.is_empty() {
        println!(
            "{}",
            console::style("您的购物车是空的，无法下单，请添加商品到购物车")
                .red()
                .bold()
        );
        std::thread::sleep(std::time::Duration::from_millis(1000));
        update_step(7);
        return;
    }

    let total: f32 = orders
        .iter()
        .map(|item| item.product.price * item.quantity as f32)
        .sum();
    let balance = update_balance(0.0);
    if balance - total >= 0.0 {
        update_balance(-total);
        println!(
            "{}",
            console::style(format!("支付成功， 共计花费{}元", total))
                .green()
                .bold()
        );
        std::thread::sleep(std::time::Duration::from_millis(1000));
        update_step(100);
    } else {
        println!("{}", console::style("余额不足，请充值！").red().bold());
        std::thread::sleep(std::time::Duration::from_millis(1000));
        update_step(4);
    }
}

fn start() {
    let products: Vec<Product> = gen_products();
    let categories = get_categories(&products);
    let mut balance = 0.00;
    // let orders: Vec<Order> = vec![];
    let mut orders: HashMap<i32, (&Product, u32)> = HashMap::new();

    'outer: loop {
        render_home();
        let input = handle_input();
        let mut step: i32 = input.trim().parse().expect("输入错误");
        loop {
            match step {
                0 => render_home(),
                1 => {
                    println!("step: {}", step);
                    render_categories(&products, &categories, |val| step = val);
                }
                2 => render_product_by_name(&products, |val| step = val),
                3 => {
                    let a = orders
                        .iter()
                        .map(|s| Order {
                            product: s.1 .0,
                            quantity: s.1 .1,
                        })
                        .collect();
                    pay(
                        a,
                        |val| {
                            balance += val;
                            balance
                        },
                        |val| step = val,
                    );
                }
                4 => {
                    render_recharge(|val| {
                        balance += val;
                        balance
                    });
                    break;
                }
                5 => {
                    render_view_balance(balance);
                    break;
                }
                6 => {
                    break 'outer;
                }
                7 => {
                    add_cart(
                        &products,
                        |val| step = val,
                        |key, val| {
                            if let Some((p, count)) = orders.get(&key) {
                                orders.insert(key, (p, val.1 + *count));
                            } else {
                                orders.insert(key, (val.0, val.1));
                            }
                            println!("{:?}", orders);
                        },
                    );
                    fn add_cart<'a>(
                        products: &'a Vec<Product>,
                        mut update_step: impl FnMut(i32),
                        mut update_orders: impl FnMut(i32, (&'a Product, u32)),
                    ) {
                        let (map, names) = render_all_products(products);
                        println!(
                            "{}",
                            console::style("请输入商品编号和购买数量，每次只能输入一组正整数")
                                .green()
                                .bold()
                        );
                        println!(
                            "{}",
                            console::style("输入格式：商品编号 购买数量， 例如26 10")
                                .green()
                                .bold()
                        );
                        let input = handle_input();
                        let item: Vec<_> = input
                            .trim()
                            .split(' ')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i32>().expect("输入格式错误"))
                            .collect();
                        println!("{:?}", item);
                        if item.len() == 2 {
                            if item[0] >= (names.len() + 1) as i32 {
                                println!(
                                    "{}",
                                    console::style("商品编号不存在, 请重新输入").red().bold()
                                );
                            } else {
                                if let Some(product) =
                                    products.iter().find(|p: &&Product| p.id as i32 == item[0])
                                {
                                    update_orders(item[0] as i32, (&product, item[1] as u32))
                                } else {
                                    println!(
                                        "{}",
                                        console::style("商品编号不存在, 请重新输入").red().bold()
                                    );
                                }
                            }
                            std::thread::sleep(std::time::Duration::from_millis(800));
                            update_step(7);
                        } else {
                            if item[0] == (names.len() + 1) as i32 {
                                update_step(100);
                                return;
                            }
                            println!(
                                "{}",
                                console::style("输入格式错误, 请重新输入").red().bold()
                            );
                            std::thread::sleep(std::time::Duration::from_millis(800));
                            update_step(7);
                        }
                    }
                }
                100 => break,
                _ => {
                    render_error_selection();
                    break;
                }
            }
        }
    }
    println!("{}", console::style("退出成功！").green().bold());
}
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:03}\t{:8}\t{:8}\t{:6}\t{:12}",
            self.id, self.name, self.category.1, self.price, self.stock
        )
    }
}
struct Order<'a> {
    product: &'a Product,
    quantity: u32,
}

fn render_recharge(mut update_balance: impl FnMut(f32) -> f32) {
    println!(
        "{}{}",
        console::style("请输入您要充值的金额").blue().bold(),
        console::style("（仅支持大于0的整数和小数）").red().bold()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<f32>() {
        Ok(val) => {
            let balance = update_balance(val);
            println!(
                "{}",
                console::style(format!("充值成功，您的余额为: {} 元", balance))
                    .bold()
                    .green()
            );
        }
        Err(_) => {
            println!(
                "{}",
                console::style("输入格式错误，请重新输入").red().bold()
            );
            render_recharge(update_balance);
        }
    }
}
fn render_error_selection() {
    println!("{}", console::style("你选择了无效的选项").red().bold());
}
fn render_view_balance(balance: f32) {
    if balance > 0.00 {
        println!(
            "{}",
            console::style(format!("您的余额为: {:.2} 元", balance))
                .blue()
                .bold()
        );
    } else {
        println!(
            "您的余额为: {} 元, {}",
            balance,
            console::style("请回到主菜单，进行充值").red().bold()
        );
    }
}

fn render_go_home(order: i32) {
    let temp = if order < 10 {
        format!("[{:}]: 返回主菜单", order)
    } else {
        format!("[{:>2}]: 返回主菜单", order)
    };
    let str = console::style(temp).bold().blue().red();
    println!("{}", str);
}

fn show_products(products: &Vec<&Product>) {
    let header = format!("产品Id\t产品名称\t产品分类\t产品价格\t库存数量");
    let line = "-".repeat(header.len() + 4);
    println!("{}", line);
    println!("{}", header);
    println!("{}", line);
    for product in products {
        println!("{}", product);
        println!("{}", line);
    }
}
fn get_total_amount(orders: Vec<Order>) -> f32 {
    orders
        .iter()
        .map(|item| item.product.price * item.quantity as f32)
        .sum()
}

fn get_categories(products: &Vec<Product>) -> HashMap<&str, &str> {
    let mut categories: HashMap<&str, &str> = HashMap::new();
    for product in products {
        categories.insert(&product.category.0, &product.category.1);
    }
    categories
}

fn get_product_by_category<'a>(products: &'a Vec<Product>, category: &str) -> Vec<&'a Product> {
    let res: Vec<&Product> = products
        .iter()
        .filter(|item| item.category.1 == category)
        .collect();

    res
}

fn get_product_by_name<'a>(products: &'a Vec<Product>, product_name: &str) -> Option<&'a Product> {
    let res = products.iter().find(|item| item.name == product_name);
    res
}
fn gen_products() -> Vec<Product> {
    let office_category = Category(String::from("Office"), String::from("办公用品"));
    let trip_category = Category(String::from("Trip"), String::from("出行"));
    let health_category = Category(String::from("Health"), String::from("健康"));
    let personal_category = Category(String::from("Personal"), String::from("个人护理"));
    let life_category = Category(String::from("Life"), String::from("生活周边"));
    let office_products = vec![
        Product {
            name: String::from("显示器"),
            category: office_category.clone(),
            price: 1.0,
            stock: 100,
            id: 1,
        },
        Product {
            name: String::from("路由器"),
            category: office_category.clone(),
            price: 2.0,
            stock: 100,
            id: 2,
        },
        Product {
            name: String::from("书写工具"),
            category: office_category.clone(),
            price: 3.0,
            stock: 100,
            id: 3,
        },
        Product {
            name: String::from("滑鼠"),
            category: office_category.clone(),
            price: 4.0,
            stock: 100,
            id: 4,
        },
        Product {
            name: String::from("小黑板"),
            category: office_category.clone(),
            price: 5.0,
            stock: 100,
            id: 5,
        },
        Product {
            name: String::from("相片打印机"),
            category: office_category.clone(),
            price: 6.0,
            stock: 100,
            id: 6,
        },
    ];

    let trip_products: Vec<Product> = vec![
        Product {
            name: String::from("车充"),
            category: trip_category.clone(),
            price: 1.0,
            stock: 100,
            id: 7,
        },
        Product {
            name: String::from("露营灯"),
            category: trip_category.clone(),
            price: 2.0,
            stock: 100,
            id: 8,
        },
        Product {
            name: String::from("旅行箱"),
            category: trip_category.clone(),
            price: 3.0,
            stock: 100,
            id: 9,
        },
        Product {
            name: String::from("眼镜"),
            category: trip_category.clone(),
            price: 4.0,
            stock: 100,
            id: 10,
        },
        Product {
            name: String::from("滑板车"),
            category: trip_category.clone(),
            price: 5.0,
            stock: 100,
            id: 11,
        },
        Product {
            name: String::from("行动电源"),
            category: trip_category.clone(),
            price: 6.0,
            stock: 100,
            id: 12,
        },
        Product {
            name: String::from("背包"),
            category: trip_category.clone(),
            price: 7.0,
            stock: 100,
            id: 13,
        },
        Product {
            name: String::from("电动打气机"),
            category: trip_category.clone(),
            price: 8.0,
            stock: 100,
            id: 14,
        },
        Product {
            name: String::from("自拍杆"),
            category: trip_category.clone(),
            price: 9.0,
            stock: 100,
            id: 15,
        },
    ];

    let health_products = vec![
        Product {
            name: String::from("筋膜枪"),
            category: health_category.clone(),
            price: 1.0,
            stock: 100,
            id: 16,
        },
        Product {
            name: String::from("体重（脂）计"),
            category: health_category.clone(),
            price: 2.0,
            stock: 100,
            id: 17,
        },
    ];
    let person_products = vec![
        Product {
            name: String::from("牙刷"),
            category: personal_category.clone(),
            price: 1.0,
            stock: 100,
            id: 18,
        },
        Product {
            name: String::from("理发器"),
            category: personal_category.clone(),
            price: 2.0,
            stock: 100,
            id: 19,
        },
        Product {
            name: String::from("护理配件"),
            category: personal_category.clone(),
            price: 3.0,
            stock: 100,
            id: 20,
        },
        Product {
            name: String::from("洗手机"),
            category: personal_category.clone(),
            price: 4.0,
            stock: 100,
            id: 21,
        },
        Product {
            name: String::from("风筒"),
            category: personal_category.clone(),
            price: 5.0,
            stock: 100,
            id: 22,
        },
        Product {
            name: String::from("挂烫机"),
            category: personal_category.clone(),
            price: 6.0,
            stock: 100,
            id: 23,
        },
    ];
    let life_products = vec![
        Product {
            name: String::from("手电筒"),
            category: life_category.clone(),
            price: 1.0,
            stock: 100,
            id: 24,
        },
        Product {
            name: String::from("电钻"),
            category: life_category.clone(),
            price: 2.0,
            stock: 100,
            id: 25,
        },
        Product {
            name: String::from("水杯"),
            category: life_category.clone(),
            price: 3.0,
            stock: 100,
            id: 26,
        },
        Product {
            name: String::from("洗车机"),
            category: life_category.clone(),
            price: 4.0,
            stock: 100,
            id: 27,
        },
        Product {
            name: String::from("测距仪"),
            category: life_category.clone(),
            price: 5.0,
            stock: 100,
            id: 10,
        },
        Product {
            name: String::from("电池"),
            category: life_category.clone(),
            price: 6.0,
            stock: 100,
            id: 11,
        },
    ];
    let mut products = vec![];
    products.push(office_products);
    products.push(trip_products);
    products.push(health_products);
    products.push(person_products);
    products.push(life_products);

    let mut rnd = rand::thread_rng();
    let mut id = 1u32;
    let mut all_products: Vec<Product> = products.into_iter().flatten().collect();
    for item in all_products.iter_mut() {
        let stock: u32 = rnd.gen_range(0..=100);
        item.stock = stock;
        let price: f32 = rnd.gen_range(0.0..=1000.0);
        item.price = (price * 100.0).round() / 100.0;
        item.id = id;
        id += 1;
    }
    all_products
}
