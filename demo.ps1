# 演示 Rust DDD 用户订单管理系统的使用 (PowerShell 版本)

$BASE_URL = "http://localhost:3000/api"
$DB_FILE = ".\data\app.db"

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Rust DDD 用户订单管理系统 - 演示" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 检查服务是否运行
Write-Host "1. 检查服务是否运行..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$BASE_URL/health" -UseBasicParsing -ErrorAction Stop
    Write-Host "✓ 服务运行正常！" -ForegroundColor Green
    $response.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10
} catch {
    Write-Host "✗ 服务未运行，请先执行：" -ForegroundColor Red
    Write-Host "  cargo run --bin backend" -ForegroundColor White
    Write-Host ""
    Write-Host "注意：在另一个终端窗口中运行服务" -ForegroundColor Gray
    exit 1
}
Write-Host ""

# 创建用户
Write-Host "2. 创建用户..." -ForegroundColor Yellow
$userBody = @{
    name = "张三"
    email = "zhangsan@example.com"
} | ConvertTo-Json
$userResponse = Invoke-WebRequest -Uri "$BASE_URL/users" -Method Post -Body $userBody -ContentType "application/json" -UseBasicParsing
$user = $userResponse.Content | ConvertFrom-Json
$user | ConvertTo-Json -Depth 10
$USER_ID = $user.id
Write-Host ""

# 获取用户
Write-Host "3. 获取用户信息..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/users/$USER_ID" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 创建订单
Write-Host "4. 为用户创建订单..." -ForegroundColor Yellow
$orderBody = @{
    user_id = $USER_ID
} | ConvertTo-Json
$orderResponse = Invoke-WebRequest -Uri "$BASE_URL/orders" -Method Post -Body $orderBody -ContentType "application/json" -UseBasicParsing
$order = $orderResponse.Content | ConvertFrom-Json
$order | ConvertTo-Json -Depth 10
$ORDER_ID = $order.id
Write-Host ""

# 添加订单项
Write-Host "5. 添加订单项..." -ForegroundColor Yellow
$itemBody1 = @{
    item = @{
        product_name = "iPhone 15"
        quantity = 2
        unit_price_yuan = 8999.0
    }
} | ConvertTo-Json -Depth 10
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/items" -Method Post -Body $itemBody1 -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 添加另一个订单项
Write-Host "6. 添加另一个订单项..." -ForegroundColor Yellow
$itemBody2 = @{
    item = @{
        product_name = "AirPods Pro"
        quantity = 1
        unit_price_yuan = 1999.0
    }
} | ConvertTo-Json -Depth 10
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/items" -Method Post -Body $itemBody2 -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 查看订单详情
Write-Host "7. 查看订单详情..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 设置折扣
Write-Host "8. 设置 10% 折扣..." -ForegroundColor Yellow
$discountBody = @{
    discount_rate = 0.1
} | ConvertTo-Json
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/discount" -Method Post -Body $discountBody -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 支付订单
Write-Host "9. 支付订单..." -ForegroundColor Yellow
$payBody = @{
    status = "pay"
} | ConvertTo-Json
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/status" -Method Post -Body $payBody -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 发货
Write-Host "10. 订单发货..." -ForegroundColor Yellow
$shipBody = @{
    status = "ship"
} | ConvertTo-Json
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/status" -Method Post -Body $shipBody -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 完成订单
Write-Host "11. 订单完成..." -ForegroundColor Yellow
$completeBody = @{
    status = "complete"
} | ConvertTo-Json
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID/status" -Method Post -Body $completeBody -ContentType "application/json" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 查看最终订单状态
Write-Host "12. 查看最终订单状态..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/orders/$ORDER_ID" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 查看所有用户
Write-Host "13. 查看所有用户..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/users" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 查看所有订单
Write-Host "14. 查看所有订单..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/orders" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

# 查看用户的订单
Write-Host "15. 查看用户的订单..." -ForegroundColor Yellow
Invoke-WebRequest -Uri "$BASE_URL/users/$USER_ID/orders" -UseBasicParsing | ForEach-Object { $_.Content | ConvertFrom-Json | ConvertTo-Json -Depth 10 }
Write-Host ""

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "演示完成！" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "数据库文件：$DB_FILE" -ForegroundColor Gray
Write-Host "你可以使用 SQLite 工具查看数据" -ForegroundColor Gray
