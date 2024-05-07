/// The variations of procedures that we can have in the Viper programming language
#[derive(Clone, Debug, PartialEq)]
pub enum ProcedureKind {
    /// Top-level procedure that is defined at program or file scope
    /// `
    /// proc main(params) {
    ///     ...
    /// }
    /// `
    TopLevel,
   
    /// Functions defined within another function
    /// `
    /// proc main() {
    ///     proc test() {
    ///         return 6;
    ///     }
    ///     ...
    /// }
    /// `
    Inline,

    /// Lambda functions
    /// `
    /// let lambda = |a, b|: i32 => {
    ///     return a + b;
    /// }
    /// `
    Lambda,
}
