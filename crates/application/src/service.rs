//! 应用服务模块
//!
//! 实现业务用例

use crate::dto::*;
use crate::error::ApplicationError;
use domain::aggregate::{Order, User};
use domain::port::{OrderRepository, UserRepository};
use domain::value_object::*;
use std::sync::Arc;

// ==================== 用户应用服务 ====================

/// 用户服务
#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    /// 创建新的用户服务
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        UserService { user_repo }
    }

    /// 创建用户
    pub async fn create_user(
        &self,
        req: CreateUserRequest,
    ) -> Result<UserResponse, ApplicationError> {
        let name = UserName::new(req.name)?;
        let email = Email::new(req.email)?;

        let user = User::new(name, email)?;
        self.user_repo.save(&user).await?;

        Ok(user.into())
    }

    /// 获取用户
    pub async fn get_user(&self, id: String) -> Result<UserResponse, ApplicationError> {
        let user_id = UserId::from_str(&id)?;
        let user = self.user_repo.find_by_id(user_id).await?;

        match user {
            Some(u) => Ok(u.into()),
            None => Err(ApplicationError::Domain(
                domain::error::DomainError::NotFound,
            )),
        }
    }

    /// 获取所有用户
    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, ApplicationError> {
        let users = self.user_repo.find_all().await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    /// 更新用户
    pub async fn update_user(
        &self,
        id: String,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, ApplicationError> {
        let user_id = UserId::from_str(&id)?;
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(domain::error::DomainError::NotFound)?;

        if let Some(name) = req.name {
            let new_name = UserName::new(name)?;
            user.update_name(new_name);
        }

        if let Some(email) = req.email {
            let new_email = Email::new(email)?;
            user.update_email(new_email);
        }

        if let Some(age) = req.age {
            let new_age = Age::new(age)?;
            user.set_age(new_age);
        }

        if let Some(rating) = req.rating {
            let new_rating = Rating::new(rating)?;
            user.set_rating(new_rating);
        }

        if let Some(note) = req.note {
            user.set_note(note);
        }

        self.user_repo.update(&user).await?;
        Ok(user.into())
    }

    /// 删除用户
    pub async fn delete_user(&self, id: String) -> Result<(), ApplicationError> {
        let user_id = UserId::from_str(&id)?;
        self.user_repo.delete(user_id).await?;
        Ok(())
    }
}

// ==================== 订单应用服务 ====================

/// 订单应用服务
#[derive(Clone)]
pub struct OrderService {
    order_repo: Arc<dyn OrderRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl OrderService {
    /// 创建新的订单服务
    pub fn new(order_repo: Arc<dyn OrderRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        OrderService {
            order_repo,
            user_repo,
        }
    }

    /// 创建订单
    pub async fn create_order(
        &self,
        req: CreateOrderRequest,
    ) -> Result<OrderResponse, ApplicationError> {
        let user_id = UserId::from_str(&req.user_id)?;

        // 验证用户存在
        if self.user_repo.find_by_id(user_id).await?.is_none() {
            return Err(ApplicationError::Domain(
                domain::error::DomainError::NotFound,
            ));
        }

        let order = Order::new(user_id)?;
        self.order_repo.save(&order).await?;

        Ok(order.into())
    }

    /// 获取订单
    pub async fn get_order(&self, id: String) -> Result<OrderResponse, ApplicationError> {
        let order_id = OrderId::from_str(&id)?;
        let order = self.order_repo.find_by_id(order_id).await?;

        match order {
            Some(o) => Ok(o.into()),
            None => Err(ApplicationError::Domain(
                domain::error::DomainError::NotFound,
            )),
        }
    }

    /// 获取用户订单
    pub async fn get_user_orders(
        &self,
        user_id: String,
    ) -> Result<Vec<OrderResponse>, ApplicationError> {
        let uid = UserId::from_str(&user_id)?;
        let orders = self.order_repo.find_by_user_id(uid).await?;
        Ok(orders.into_iter().map(|o| o.into()).collect())
    }

    /// 获取所有订单
    pub async fn get_all_orders(&self) -> Result<Vec<OrderResponse>, ApplicationError> {
        let orders = self.order_repo.find_all().await?;
        Ok(orders.into_iter().map(|o| o.into()).collect())
    }

    /// 添加订单项
    pub async fn add_order_item(
        &self,
        order_id: String,
        req: AddOrderItemRequest,
    ) -> Result<OrderResponse, ApplicationError> {
        let oid = OrderId::from_str(&order_id)?;
        let mut order = self
            .order_repo
            .find_by_id(oid)
            .await?
            .ok_or(domain::error::DomainError::NotFound)?;

        let quantity = Quantity::new(req.item.quantity)?;
        let unit_price = Money::from_yuan(req.item.unit_price_yuan)?;

        let item = OrderItem::new(req.item.product_name, quantity, unit_price);
        order.add_item(item);

        self.order_repo.update(&order).await?;
        Ok(order.into())
    }

    /// 设置折扣
    pub async fn set_discount(
        &self,
        order_id: String,
        req: SetDiscountRequest,
    ) -> Result<OrderResponse, ApplicationError> {
        let oid = OrderId::from_str(&order_id)?;
        let mut order = self
            .order_repo
            .find_by_id(oid)
            .await?
            .ok_or(domain::error::DomainError::NotFound)?;

        let discount_rate = DiscountRate::new(req.discount_rate)?;
        order.set_discount_rate(discount_rate);

        self.order_repo.update(&order).await?;
        Ok(order.into())
    }

    /// 更新订单状态
    pub async fn update_order_status(
        &self,
        order_id: String,
        req: UpdateOrderStatusRequest,
    ) -> Result<OrderResponse, ApplicationError> {
        let oid = OrderId::from_str(&order_id)?;
        let mut order = self
            .order_repo
            .find_by_id(oid)
            .await?
            .ok_or(domain::error::DomainError::NotFound)?;

        match req.status.as_str() {
            "pay" => order.pay()?,
            "ship" => order.ship()?,
            "complete" => order.complete()?,
            "cancel" => order.cancel()?,
            _ => return Err(ApplicationError::Validation("无效的状态操作".into())),
        }

        self.order_repo.update(&order).await?;
        Ok(order.into())
    }

    /// 删除订单
    pub async fn delete_order(&self, id: String) -> Result<(), ApplicationError> {
        let oid = OrderId::from_str(&id)?;
        self.order_repo.delete(oid).await?;
        Ok(())
    }
}
