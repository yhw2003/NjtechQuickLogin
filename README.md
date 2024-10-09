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

很可惜，尽管确实存在着大量的misp64*架构的路由器，但是rust stabe工具链并不支持它们，你需要修改`rust-toolchain.toml`切换到nightly工具链才能构建
```toml
# rust-toolchain.toml
[toolchain]
channel = "1.80.1"
```
并且我不得不将它从ci移除

你可以前往[Actions](https://github.com/yhw2003/NjtechQuickLogin/actions)下载构建的版本，也可看fork仓库自行发起actions构建。
并不是所有的target都经过测试，如果你有无构建的target，请提issue。

rust语言的musl C库是静态连接的，这意味着musl C库构建的二进制文件可以在任何系统上运。

TODO:
    - ci/bugfix: 自动发布二进制到release