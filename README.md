# TomatoOS
跟个西红柿的系统
是x86-64的系统
# 环境配置
需要你自己去配置好Rust的环境  
配置好后运行make fixbuild  
运行完成后记得把Cargo.toml中的dependencies下的xbuild和bootimage删掉

# 构建
直接运行make  
运行完成后需要用QEMU运行编译完成的bin文件  

# 运行
如`qemu-system-x86_64 -drive format=raw,file=bootimage-tomato_os.bin`  
请根据自己的实际情况合理变更  

# TODO
- [ ] 键盘输入
- [ ] 多线程
- [ ] 文件系统
- [ ] 用户系统
- [ ] vga TUI界面
- [ ] 显卡驱动
- [ ] 简易桌面
- [ ] 终端程序
- [ ] mcode编译器
- [ ] 网卡驱动，网络连接