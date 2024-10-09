一个简单的njtech-home自动链接/保活工具
专为路由器等设备设计，0外部依赖，自守护
（众所周知你的openwrt大概率没有systemd）


``` shell
// 启动
./NjtechQuickLogin -c "config.toml"
```

```toml
# toml配置
user = "***"
password = "***"
isp = "***" #移动：cmcc，电信：telnet
force_start = true #首次链接失败仍然启动，用于保活已连接设备
err_log = "./err.log" # 错误日志输出(stderr)
log = "./out.log" # 日志输出(stdout)
```

```shell
//关闭 (kill掉就好了)
ps -ef | grep -i njtech
kill ${pid}
```

为了方便你在`init.d`中配置自启，程序还提供了选择pwd的方法
```shell
./NjtechQuickLogin -c "config.toml" --pwd "/root/njtech-quick-login/"
```