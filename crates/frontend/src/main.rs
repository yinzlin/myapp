//! Dioxus 前端应用入口
//! 
//! 这是前端应用的启动入口，使用 Dioxus 框架构建响应式 UI。

// 导入 Dioxus 核心模块
use dioxus::prelude::*;

// 模块导入：UI 组件定义
mod components;
// 模块导入：页面组件定义
mod pages;
// 模块导入：状态管理
mod state;

/// 主函数：启动 Dioxus 应用
fn main() {
    // 启动 Dioxus 应用，渲染 App 组件
    dioxus::launch(App);
}

/// 根组件：应用的最顶层组件
#[component]
fn App() -> Element {
    // 使用 RSX 宏定义 UI 结构
    rsx! {
        // 容器 div
        div {
            // 标题元素
            h1 { "Rust 全栈应用" }
            // 段落元素
            p { "欢迎使用 Dioxus 前端框架" }
        }
    }
}
