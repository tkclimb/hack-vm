use crate::ast::{Segment, Stmt};

const word_size: i32 = 4;
const constant_segment_size: i32 = 10;
const local_segment_size: i32 = 100;
const static_segment_size: i32 = 100;

pub struct Codegen<'a> {
  pub asm_list: Vec<String>,
  stmt_list: &'a Vec<Stmt>,
  indent: i32,
}

impl<'a> Codegen<'a> {
  pub fn new(stmt_list: &'a Vec<Stmt>) -> Self {
    Codegen {
      asm_list: Vec::new(),
      stmt_list: stmt_list,
      indent: 0,
    }
  }

  pub fn run(&mut self) {
    self.gen_init();

    for stmt in self.stmt_list {
      self.set(&format!("# {:?}", stmt));
      self.gen(stmt);
    }

    self.gen_end();
  }

  pub fn gen_init(&mut self) {
    self.set(".section __DATA,__data");
    // constant segment
    self.set("_constant:");
    for i in 0..constant_segment_size {
      self.set(&format!(".long {}", i));
    }
    self.set("");

    // local segment
    self.set(&format!(
      ".zerofill __DATA,__bss,_local,{},4",
      local_segment_size * word_size
    ));
    // local static
    self.set(&format!(
      ".zerofill __DATA,__bss,_static,{},4",
      static_segment_size * word_size
    ));
    self.set("");

    // TEXT section
    self.set(".section __TEXT,__text");
    self.set(".globl _main");
    self.set("");
    self.set("_main:");
    self.inc_indent();
    self.set("push %rbp");
    self.set("mov %rsp, %rbp");
  }

  pub fn gen_end(&mut self) {
    self.set("mov $0, %rax");
    self.set("pop %rbp");
    self.set("ret");
  }

  fn gen(&mut self, stmt: &'a Stmt) {
    match stmt {
      Stmt::Push { segment, index } => {
        // String::new();
        let offset = index * word_size;
        let instr = match segment {
          Segment::Local => format!("push _local+{}(%rip)", offset),
          Segment::Constant => format!("push _constant+{}(%rip)", offset),
          _ => {
            panic!("not supported...");
          }
        };
        self.set(&instr);
      }

      Stmt::Pop { segment, index } => {
        let offset = index * word_size;
        let instr = match segment {
          Segment::Local => format!("pop _local+{}(%rip)", offset),
          Segment::Constant => format!("pop _constant+{}(%rip)", offset),
          _ => {
            panic!("not supported...");
          }
        };
        self.set(&instr);
      }

      Stmt::Add => {
        self.set("pop %rax");
        self.set("pop %rbx");
        self.set("add %rbx, %rax");
        self.set("push %rax");
      }

      Stmt::Sub => {
        self.set("pop %rax");
        self.set("pop %rbx");
        self.set("sub %rbx, %rax");
        self.set("push %rax");
      }
    }
  }

  fn set(&mut self, cmd: &str) {
    let mut indent = String::new();
    for _ in 0..self.indent {
      indent += " ";
    }
    self.asm_list.push(indent + cmd);
  }

  fn inc_indent(&mut self) {
    self.indent += 2;
  }

  fn dec_indent(&mut self) {
    self.indent -= 2;
  }
}
