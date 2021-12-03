# deepcis-os

# What is this ? 
このプロジェクトはblog-osを基調として自作OSをRustによって実装する個人開発プロジェクトです。

## How to use
qemuをインストールしたのちに
'''
./run-qemu
'''
を実行すると動きます。


ただし、OVMF.fdファイルが /usr/share/OVMF/x64/OVMF.fd にない場合は
rum-qemuファイルの中の
-bios /usr/share/OVMF/x64/OVMF.fd の部分を、ルートディレクトリにあOVMF-pure-efi.fd に変更すれば動きます。


参考元サイト:
https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/01-minimal-kernel/index.md 

https://gifnksm.hatenablog.jp/archive/category/%E3%82%BC%E3%83%AD%E3%81%8B%E3%82%89%E3%81%AEOS%E8%87%AA%E4%BD%9C%E5%85%A5%E9%96%80  