/// Proposal: Remove dyn-clone dependency from cspsolver
/// 
/// ## Current Situation
/// - dyn-clone is used to make Box<dyn Prune> cloneable
/// - This is needed because Propagators derives Clone
/// - Space cloning during search requires cloning Propagators
/// 
/// ## Key Insights
/// 1. **All propagators are stateless** - they only store variable references
/// 2. **Propagators never mutate during pruning** - &mut self is unused
/// 3. **Space already has save/restore for variable domains**
/// 4. **Search branching only needs to restore variable domains, not propagators**
/// 
/// ## Proposed Solution
/// 
/// ### Option 1: Remove Clone from Propagators (Recommended)
/// ```rust
/// // Change Propagators to not derive Clone
/// #[derive(Debug, Default)] // Remove Clone
/// pub struct Propagators {
///     state: Vec<Box<dyn Prune>>,
///     // ... rest unchanged
/// }
/// 
/// // Update Space to use save/restore instead of clone
/// impl Space {
///     fn branch(&self, constraint: PropId) -> Space {
///         let mut new_space = Space {
///             vars: self.vars.clone(), // Only clone variables
///             props: self.props, // Move propagators (don't clone)
///         };
///         new_space.props.add_constraint(constraint);
///         new_space
///     }
/// }
/// ```
/// 
/// ### Option 2: Make Prune trait take &self instead of &mut self
/// ```rust
/// pub trait Prune: DynClone + Send + Sync + std::fmt::Debug {
///     fn prune(&self, ctx: &mut Context) -> Option<()>; // &self instead of &mut self
///     fn list_trigger_vars(&self) -> impl Iterator<Item = VarId>;
/// }
/// ```
/// Then propagators can derive Copy and cloning becomes free.
/// 
/// ### Option 3: Use Rc<dyn Prune> instead of Box<dyn Prune>
/// ```rust
/// pub struct Propagators {
///     state: Vec<Rc<dyn Prune>>, // Shared references
///     // ... rest unchanged
/// }
/// ```
/// This makes cloning O(1) instead of requiring deep clones.
/// 
/// ## Recommended Implementation: Option 1
/// 
/// 1. **Remove Clone from Propagators**
/// 2. **Update search to use save/restore instead of clone**
/// 3. **Remove dyn-clone dependency entirely**
/// 
/// ### Changes needed:
/// 
/// 1. **src/props/mod.rs**:
///    - Remove `clone_trait_object!(Prune);`
///    - Remove `Clone` from `Propagators` derive
///    - Remove `DynClone` from `Prune` trait
/// 
/// 2. **src/search/branch.rs**:
///    - Replace `space.clone()` with save/restore pattern
///    - Use `space.save_state()` before branching
///    - Use `space.restore_state()` for backtracking
/// 
/// 3. **Cargo.toml**:
///    - Remove `dyn-clone` dependency
/// 
/// ## Benefits
/// - **Smaller dependency tree** - no dyn-clone
/// - **Better performance** - save/restore is O(variables) vs O(everything)  
/// - **Cleaner design** - stateless propagators don't need cloning
/// - **Memory efficiency** - shared propagators instead of deep copies
/// 
/// ## Testing
/// All existing tests should pass since propagators are stateless.
/// The search behavior is identical, just using save/restore instead of clone.