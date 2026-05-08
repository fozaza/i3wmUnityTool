use crate::cmd;
use anyhow::Ok;
use crossterm::{
    QueueableCommand, cursor,
    event::{Event, poll, read},
    style::{self, Stylize},
    terminal,
};
use std::{io::Write, process::exit, time::Duration};

#[derive(Debug)]
pub struct TuiApp {
    pub size_term: (u16, u16),
    pub pos: (u16, u16),
    pub pos_workspace: (u16, u16),
    pub max_pos_workspace: (u16, u16),
    pub stdout: std::io::Stdout,
    pub index_chose: usize,
    pub index_old: usize,
    pub lock_pos_workspace: (u16, u16),
}

const POS_WORKSAPCE_DEFINE: (u16, u16) = (5, 3);

impl TuiApp {
    pub fn init() -> anyhow::Result<Self> {
        Ok(Self {
            size_term: (0, 0),
            pos: (0, 0),
            pos_workspace: POS_WORKSAPCE_DEFINE,
            max_pos_workspace: (0, 0),
            stdout: std::io::stdout(),
            index_chose: 0,
            index_old: 0,
            lock_pos_workspace: (0, 0),
        })
    }

    pub fn size_check(&mut self) -> anyhow::Result<()> {
        let size_term_now = terminal::size()?;
        if size_term_now != self.size_term {
            self.size_term = size_term_now;
            self.max_pos_workspace = (size_term_now.0 - 6, size_term_now.1 - 3);
            self.pos_workspace = POS_WORKSAPCE_DEFINE;

            self.stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(self.pos.0, self.pos.1))?
                .flush()?;
        } else {
            return Ok(());
        }

        for i in 0..self.size_term.0 {
            for j in 0..self.size_term.1 {
                if i == 0
                    || i == 1
                    || i == self.size_term.0 - 1
                    || i == self.size_term.0 - 2
                    || (i > 3
                        && i <= self.size_term.0 - 5
                        && j != 0
                        && j != 1
                        && j != size_term_now.1 - 1
                        && j != size_term_now.1 - 2)
                {
                    continue;
                }
                self.stdout
                    .queue(cursor::MoveTo(i, j))?
                    .queue(style::PrintStyledContent("*".blue()))?
                    .flush()?;
            }
        }

        for i in 6..8 as u16 {
            for j in 0..self.size_term.0 {
                if j == 0 || j == 1 || j == 2 || j == 3 || j > self.size_term.0 - 5 {
                    continue;
                }
                self.stdout
                    .queue(cursor::MoveTo(j, i))?
                    .queue(style::PrintStyledContent("-".blue()))?
                    .flush()?;
            }
        }

        let title = self.format_ctx("BackGound chosse");
        self.set_center(&title)?.println(&title)?;

        let app_size = self
            .format_ctx(format!("app size x:{} y:{}", self.size_term.0, self.size_term.1).as_str());
        self.set_center(&app_size)?.println(&app_size)?;

        self.stdout.queue(cursor::MoveDown(4))?.flush()?;
        self.pos_workspace.1 += 4;
        let text = self.format_ctx("Image list");
        self.set_center(&text)?.println(&text)?;

        self.lock_pos_workspace = self.pos_workspace;
        self.pos_workspace.1 = self.max_pos_workspace.1 - 1;
        let bottom =
            format!("Esc : Exit program  k/j : up/down  Enter : to chosse image  r : reload feh");
        self.set_center(&bottom)?;
        self.update_pos(self.pos_workspace.0, self.pos_workspace.1)?;
        self.stdout
            .queue(style::PrintStyledContent(bottom.green()))?;
        self.set_pos_lock().update_pos_now()?;

        self.update(true)?;
        //self.list_image()?;
        Ok(())
    }

    pub fn event(&mut self) -> anyhow::Result<()> {
        if poll(Duration::from_mins(500))? {
            match read()? {
                Event::Key(event) => {
                    // self.pos_old.0 = position()?.0;
                    match event.code.to_string().as_str() {
                        "Esc" => {
                            //cmd_lib::run_cmd!(clear)?;
                            self.stdout
                                .queue(terminal::Clear(terminal::ClearType::All))?
                                .queue(cursor::MoveTo(0, 0))?
                                .flush()?;
                            exit(0);
                        }
                        "j" => self.roll('j')?,
                        "k" => self.roll('k')?,
                        "Enter" => self.image_pick()?,
                        "r" => cmd::sh_reload_feh()?,
                        _ => {}
                    }
                }
                _ => {}
            }
            self.stdout.flush()?;
        }
        Ok(())
    }

    pub fn image_pick(&mut self) -> anyhow::Result<()> {
        let image_vec = cmd::image_str_vec(".config/bgrunner/backgrounds")?;
        cmd::sh_overrive_file(&image_vec[self.index_old])?;
        self.roll('j')?;
        // self.update()?;
        Ok(())
    }

    fn roll(&mut self, input: char) -> anyhow::Result<()> {
        let image_vec = cmd::image_str_vec(".config/bgrunner/backgrounds")?;

        match input {
            'k' => {
                if image_vec.len() - 1 >= self.index_chose + 1 {
                    self.index_chose += 1;
                } else {
                    self.index_chose = 0;
                }
                return Ok(());
            } //up
            'j' => {
                match self.index_chose.checked_sub(1) {
                    Some(v) => self.index_chose = v,
                    None => self.index_chose = image_vec.len() - 1,
                }
                return Ok(());
            } //down
            _ => return Ok(()),
        }
    }

    pub fn update(&mut self, run_now: bool) -> anyhow::Result<()> {
        if self.index_chose == self.index_old && !run_now {
            return Ok(());
        }

        let image_vec = cmd::image_str_vec(".config/bgrunner/backgrounds")?;
        let mut image_chosse = cmd::read_file()?;
        let mut image_char = image_chosse.chars();
        for _ in 0.."./backgrounds/".len() {
            image_char.next();
        }
        image_chosse = image_char.as_str().to_string();

        for i in 0..image_vec.len() {
            for _ in 0..image_vec[i].len() + 10 {
                self.print(" ")?;
            }
            self.println("")?;
        }
        self.set_pos_lock().update_pos_now()?;

        for i in 0..image_vec.len() {
            if self.index_chose == i {
                self.print(" >>> ")?;
                if image_vec[i].trim() == image_chosse.trim() {
                    self.print(&image_vec[i])?;
                    self.println(" <:C")?;
                    continue;
                }
                self.println(&image_vec[i])?;
                continue;
            }
            self.print(&image_vec[i])?;
            if image_vec[i].trim() == image_chosse.trim() {
                self.println(" <:C")?;
                continue;
            }
            self.println("")?;
        }
        self.index_old = self.index_chose;
        self.pos_workspace = self.lock_pos_workspace;
        self.update_pos_now()?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn format_ctx(&self, org_ctx: &str) -> String {
        if org_ctx.len() < self.max_pos_workspace.1 as usize {
            return org_ctx.to_string();
        }

        let mut new_ctx = String::new();
        let org_char: Vec<char> = org_ctx.chars().collect();
        for i in 0..self.max_pos_workspace.0 as usize - 3 {
            new_ctx.push(org_char[i]);
        }
        format!("{new_ctx}...")
    }

    pub fn set_center(&mut self, org_ctx: &str) -> anyhow::Result<&mut Self> {
        if org_ctx.len() >= self.max_pos_workspace.0 as usize {
            return Ok(self);
        }

        let helf_max_pos_x = self.max_pos_workspace.0 >> 1;
        let helf_pos_x = org_ctx.len() as u16 >> 1;
        let center_pos = helf_max_pos_x - helf_pos_x; // center_pos
        self.update_pos(center_pos, self.pos_workspace.1)?;
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn print(&mut self, ctx: &str) -> anyhow::Result<&mut Self> {
        self.stdout.queue(style::Print(ctx))?.flush()?;
        self.pos_workspace.0 += ctx.len() as u16;
        self.logic_new_line();
        Ok(self)
    }

    pub fn println(&mut self, ctx: &str) -> anyhow::Result<&mut Self> {
        self.stdout.queue(style::Print(ctx))?.flush()?;
        self.pos_workspace.1 += 1;
        self.logic_new_line();
        self.update_pos(POS_WORKSAPCE_DEFINE.0, self.pos_workspace.1)?;
        Ok(self)
    }

    fn logic_new_line(&mut self) {
        if self.pos_workspace.0 >= self.max_pos_workspace.0 {
            self.pos_workspace.0 = POS_WORKSAPCE_DEFINE.0;
            self.pos_workspace.1 += 1;
        }
        if self.pos_workspace.1 >= self.max_pos_workspace.1 {
            self.pos_workspace = POS_WORKSAPCE_DEFINE;
        }
    }

    fn update_pos_now(&mut self) -> anyhow::Result<()> {
        self.stdout
            .queue(cursor::MoveTo(self.pos_workspace.0, self.pos_workspace.1))?
            .flush()?;
        Ok(())
    }

    #[allow(dead_code)]
    fn update_pos(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        self.stdout.queue(cursor::MoveTo(x, y))?.flush()?;
        self.pos_workspace.0 = x;
        self.pos_workspace.1 = y;
        Ok(())
    }

    fn set_pos_lock(&mut self) -> &mut Self {
        self.pos_workspace = self.lock_pos_workspace;
        self
    }

    // fn clear_line(&mut self, line: u16) -> anyhow::Result<()> {
    //     self.update_pos(line, self.pos_workspace.1)?;
    //     for _ in 0..self.max_pos_workspace.1 {
    //         self.print(" ")?;
    //     }
    //     self.set_pos_lock().update_pos_now()?;
    //     Ok(())
    // }
}
