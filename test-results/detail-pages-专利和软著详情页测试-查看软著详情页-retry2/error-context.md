# Page snapshot

```yaml
- generic [ref=e5]:
  - heading "专利助手" [level=2] [ref=e6]
  - paragraph [ref=e7]: 智能专利说明书生成系统
  - generic [ref=e8]:
    - generic [ref=e9]:
      - generic [ref=e10]: "* 用户名"
      - textbox "* 用户名" [ref=e14]:
        - /placeholder: 请输入用户名
        - text: admin
    - generic [ref=e15]:
      - generic [ref=e16]: "* 密码"
      - generic [ref=e19]:
        - textbox "* 密码" [active] [ref=e20]:
          - /placeholder: 请输入密码
          - text: admin123
        - img [ref=e23] [cursor=pointer]
    - button "登录" [ref=e28] [cursor=pointer]:
      - generic [ref=e29]: 登录
    - separator [ref=e30]
    - generic [ref=e31]:
      - paragraph [ref=e32]: 测试账号
      - alert [ref=e33]:
        - img [ref=e35]
        - generic [ref=e37]:
          - generic [ref=e38]: 测试账号信息
          - paragraph [ref=e39]:
            - generic [ref=e40]:
              - paragraph [ref=e41]:
                - strong [ref=e42]: 用户名：
                - text: admin
              - paragraph [ref=e43]:
                - strong [ref=e44]: 密码：
                - text: admin123
            - button "一键填充" [ref=e45] [cursor=pointer]:
              - generic [ref=e46]: 一键填充
```