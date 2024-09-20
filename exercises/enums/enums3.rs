enum Message {
    ChangeColor(u8, u8, u8), // 颜色变体，包含三个 u8
    Echo(String),            // Echo 变体，包含一个字符串
    Move(Point),             // Move 变体，包含一个 Point 结构体
    Quit,                    // Quit 变体，不包含任何数据
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // 处理不同的消息变体
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));  // 改变颜色
        state.process(Message::Echo(String::from("hello world")));  // 回显消息
        state.process(Message::Move(Point { x: 10, y: 15 }));  // 移动位置
        state.process(Message::Quit);  // 退出

        assert_eq!(state.color, (255, 0, 255));  // 检查颜色是否改变
        assert_eq!(state.position.x, 10);  // 检查位置x是否改变
        assert_eq!(state.position.y, 15);  // 检查位置y是否改变
        assert_eq!(state.quit, true);  // 检查退出状态
        assert_eq!(state.message, "hello world");  // 检查消息是否正确回显
    }
}
