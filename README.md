# AUTO-GIT

- 为什么有这个

当新建一个项目时，通常使用如下工作流

- 新建本地仓库

- 在浏览器里新建github仓库

- 本地链接远程仓库

- 推送

即使使用了`gh cli`整个过程也比较繁琐，现在想通过这个程序将过程优化成

```shell
auto-git --new project-name --public # --private
# 自动新建远程仓库，新建本地目录，并链接二者

auto-git --push  --public # --private
# 自动新建一个与当前目录同名的远程仓库，并把本地内容推送上去
```

