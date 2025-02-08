#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ## AST Contents
///
/// Quote: the char ','
///
/// Print: the char 'p'
///
/// Add: the char 'a'
///
/// Equal: the char '='
///
/// Insert: the char 'i'
///
/// Delete: the char 'd'
///
/// Exclamation: the char '!'
///
/// Write: the char 'w'
///
/// Number: Contains u64. for Line number
///
/// Quit: the char 'q'
///
pub enum Ast {
    /// Quote: the char ','
    Quote,
    /// Print: the char 'p'
    Print,
    /// Add: the char 'a'
    Add,
    /// Equal: the char '='
    Equal,
    /// Insert: the char 'i'
    Insert,
    /// Delete: the char 'd'
    Delete,
    /// Exclamation: the char '!'
    Exclamation,
    /// Nothing state
    Normal,
    /// Write: the char 'w'
    Write,
    /// Number: Contains u64. for Line number
    Number(u64),
    /// Quit: the char 'q'
    Quit,
}
