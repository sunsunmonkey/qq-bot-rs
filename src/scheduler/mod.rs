use proc_qq::re_exports::ricq::msg::elem::{At, Text};
use proc_qq::re_exports::ricq::Client;
use proc_qq::scheduler;
use proc_qq::{
    re_exports::ricq::msg::MessageChain, scheduler_job, MessageChainAppendTrait, Scheduler,
};
use rand::random;

use std::sync::Arc;
/// 每1分钟发送一次 Hello
#[scheduler_job(cron = "0 0/1 * * * ?")]
async fn handle_scheduler(c: Arc<Client>) -> anyhow::Result<()> {
    let user_index: usize = if random() { 1 } else { 0 };
    let user_arr = [1093918683, 3331598351];
    let content_arr = [" 抽查", " CC", " 尼玛死了", " 卷尼玛呢"];
    let content_index: usize = random::<usize>() % 4;

    let chain = MessageChain::default()
        .append(At::new(user_arr[user_index]))
        .append(Text::new(content_arr[content_index].to_string()));

    c.send_group_message(895192790, chain)
        .await
        .expect("send error");
    Ok(())
}

/// scheduler
pub fn scheduler() -> Scheduler {
    scheduler!("hello_jobs", handle_scheduler)
}
