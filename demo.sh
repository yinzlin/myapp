#!/bin/bash

# 演示 Rust DDD 用户订单管理系统的使用

BASE_URL="http://localhost:3000/api"
DB_FILE="./data/app.db"

echo "====================================="
echo "Rust DDD 用户订单管理系统 - 演示"
echo "====================================="
echo ""

# 检查服务是否运行
echo "1. 检查服务是否运行..."
if curl -s "$BASE_URL/health" > /dev/null; then
    echo "✓ 服务运行正常！"
    curl -s "$BASE_URL/health" | python3 -m json.tool
else
    echo "✗ 服务未运行，请先执行："
    echo "  cargo run --bin backend"
    echo ""
    echo "注意：在 Windows 上，请使用 PowerShell 或直接在后台运行服务"
    exit 1
fi
echo ""

# 创建用户
echo "2. 创建用户..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/users" \
    -H "Content-Type: application/json" \
    -d '{
        "name": "张三",
        "email": "zhangsan@example.com"
    }')

echo "$USER_RESPONSE" | python3 -m json.tool

# 提取用户ID
USER_ID=$(echo "$USER_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('id'))")
echo ""

# 获取用户
echo "3. 获取用户信息..."
curl -s "$BASE_URL/users/$USER_ID" | python3 -m json.tool
echo ""

# 创建订单
echo "4. 为用户创建订单..."
ORDER_RESPONSE=$(curl -s -X POST "$BASE_URL/orders" \
    -H "Content-Type: application/json" \
    -d "{
        \"user_id\": \"$USER_ID\"
    }")

echo "$ORDER_RESPONSE" | python3 -m json.tool

# 提取订单ID
ORDER_ID=$(echo "$ORDER_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('id'))")
echo ""

# 添加订单项
echo "5. 添加订单项..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/items" \
    -H "Content-Type: application/json" \
    -d '{
        "item": {
            "product_name": "iPhone 15",
            "quantity": 2,
            "unit_price_yuan": 8999.0
        }
    }' | python3 -m json.tool
echo ""

# 添加另一个订单项
echo "6. 添加另一个订单项..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/items" \
    -H "Content-Type: application/json" \
    -d '{
        "item": {
            "product_name": "AirPods Pro",
            "quantity": 1,
            "unit_price_yuan": 1999.0
        }
    }' | python3 -m json.tool
echo ""

# 查看订单详情
echo "7. 查看订单详情..."
curl -s "$BASE_URL/orders/$ORDER_ID" | python3 -m json.tool
echo ""

# 设置折扣
echo "8. 设置 10% 折扣..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/discount" \
    -H "Content-Type: application/json" \
    -d '{
        "discount_rate": 0.1
    }' | python3 -m json.tool
echo ""

# 支付订单
echo "9. 支付订单..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/status" \
    -H "Content-Type: application/json" \
    -d '{
        "status": "pay"
    }' | python3 -m json.tool
echo ""

# 发货
echo "10. 订单发货..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/status" \
    -H "Content-Type: application/json" \
    -d '{
        "status": "ship"
    }' | python3 -m json.tool
echo ""

# 完成订单
echo "11. 订单完成..."
curl -s -X POST "$BASE_URL/orders/$ORDER_ID/status" \
    -H "Content-Type: application/json" \
    -d '{
        "status": "complete"
    }' | python3 -m json.tool
echo ""

# 查看最终订单状态
echo "12. 查看最终订单状态..."
curl -s "$BASE_URL/orders/$ORDER_ID" | python3 -m json.tool
echo ""

# 查看所有用户
echo "13. 查看所有用户..."
curl -s "$BASE_URL/users" | python3 -m json.tool
echo ""

# 查看所有订单
echo "14. 查看所有订单..."
curl -s "$BASE_URL/orders" | python3 -m json.tool
echo ""

# 查看用户的订单
echo "15. 查看用户的订单..."
curl -s "$BASE_URL/users/$USER_ID/orders" | python3 -m json.tool
echo ""

echo "====================================="
echo "演示完成！"
echo "====================================="
echo ""
echo "数据库文件：$DB_FILE"
echo "你可以使用 SQLite 工具查看数据："
echo "  sqlite3 $DB_FILE"
