//! API 演示客户端
//!
//! 演示如何使用我们的用户订单管理系统 API

use application::dto::*;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    let base_url = "http://localhost:3000/api";

    println!("=====================================");
    println!("Rust DDD 用户订单管理系统 - 演示");
    println!("=====================================");
    println!();

    // 1. 检查健康状态
    println!("1. 检查服务状态...");
    let health_resp = client.get(format!("{}/health", base_url)).send().await?;
    if health_resp.status().is_success() {
        println!("✓ 服务运行正常！");
    } else {
        println!("✗ 服务未运行，请先执行 'cargo run --bin backend'");
        return Ok(());
    }
    println!();

    // 2. 创建用户
    println!("2. 创建用户...");
    let user_req = CreateUserRequest {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };
    let user_resp: UserResponse = client
        .post(format!("{}/users", base_url))
        .json(&user_req)
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 创建成功！用户ID: {}", user_resp.id);
    println!("    用户信息: {:?}", user_resp);
    let user_id = user_resp.id.clone();
    println!();

    // 3. 获取用户信息
    println!("3. 获取用户信息...");
    let get_user: UserResponse = client
        .get(format!("{}/users/{}", base_url, user_id))
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 用户信息: {:?}", get_user);
    println!();

    // 4. 创建订单
    println!("4. 创建订单...");
    let order_req = CreateOrderRequest {
        user_id: user_id.clone(),
    };
    let order_resp: OrderResponse = client
        .post(format!("{}/orders", base_url))
        .json(&order_req)
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 订单创建成功！订单ID: {}", order_resp.id);
    let order_id = order_resp.id.clone();
    println!();

    // 5. 添加订单项 1
    println!("5. 添加订单项 1...");
    let item1 = AddOrderItemRequest {
        item: OrderItemRequest {
            product_name: "iPhone 15".to_string(),
            quantity: 2,
            unit_price_yuan: 8999.0,
        },
    };
    let _ = client
        .post(format!("{}/orders/{}/items", base_url, order_id))
        .json(&item1)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 订单项添加成功！");
    println!();

    // 6. 添加订单项 2
    println!("6. 添加订单项 2...");
    let item2 = AddOrderItemRequest {
        item: OrderItemRequest {
            product_name: "AirPods Pro".to_string(),
            quantity: 1,
            unit_price_yuan: 1999.0,
        },
    };
    let _ = client
        .post(format!("{}/orders/{}/items", base_url, order_id))
        .json(&item2)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 订单项添加成功！");
    println!();

    // 7. 设置折扣
    println!("7. 设置 10% 折扣...");
    let discount_req = SetDiscountRequest { discount_rate: 0.1 };
    let _ = client
        .post(format!("{}/orders/{}/discount", base_url, order_id))
        .json(&discount_req)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 折扣设置成功！");
    println!();

    // 8. 支付订单
    println!("8. 支付订单...");
    let pay_req = UpdateOrderStatusRequest {
        status: "pay".to_string(),
    };
    let _ = client
        .post(format!("{}/orders/{}/status", base_url, order_id))
        .json(&pay_req)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 订单已支付！");
    println!();

    // 9. 发货
    println!("9. 订单发货...");
    let ship_req = UpdateOrderStatusRequest {
        status: "ship".to_string(),
    };
    let _ = client
        .post(format!("{}/orders/{}/status", base_url, order_id))
        .json(&ship_req)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 订单已发货！");
    println!();

    // 10. 完成订单
    println!("10. 完成订单...");
    let complete_req = UpdateOrderStatusRequest {
        status: "complete".to_string(),
    };
    let _ = client
        .post(format!("{}/orders/{}/status", base_url, order_id))
        .json(&complete_req)
        .send()
        .await?
        .json::<OrderResponse>()
        .await?;
    println!("  ✓ 订单完成！");
    println!();

    // 11. 查看最终订单
    println!("11. 查看最终订单...");
    let final_order: OrderResponse = client
        .get(format!("{}/orders/{}", base_url, order_id))
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 订单详情: {:?}", final_order);
    println!();

    // 12. 查看用户的所有订单
    println!("12. 查看用户的所有订单...");
    let user_orders: Vec<OrderResponse> = client
        .get(format!("{}/users/{}/orders", base_url, user_id))
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 用户订单数量: {}", user_orders.len());
    println!();

    // 13. 查看所有用户
    println!("13. 查看所有用户...");
    let all_users: Vec<UserResponse> = client
        .get(format!("{}/users", base_url))
        .send()
        .await?
        .json()
        .await?;
    println!("  ✓ 用户总数: {}", all_users.len());
    println!();

    println!("=====================================");
    println!("演示完成！✓");
    println!("=====================================");
    println!();
    println!("数据库文件: ./data/app.db");

    Ok(())
}
