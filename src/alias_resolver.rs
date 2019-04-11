use common::*;
use CompilationErrorKind::*;

pub struct AliasResolver<'a, 'b>
where
  'a: 'b,
{
  aliases: &'b Map<&'a str, Alias<'a>>,
  recipes: &'b Map<&'a str, Recipe<'a>>,
  alias_tokens: &'b Map<&'a str, Token<'a>>,
}

impl<'a: 'b, 'b> AliasResolver<'a, 'b> {
  pub fn resolve_aliases(
    aliases: &Map<&'a str, Alias<'a>>,
    recipes: &Map<&'a str, Recipe<'a>>,
    alias_tokens: &Map<&'a str, Token<'a>>,
  ) -> CompilationResult<'a, ()> {
    let resolver = AliasResolver {
      aliases,
      recipes,
      alias_tokens,
    };

    resolver.resolve()?;

    Ok(())
  }

  fn resolve(&self) -> CompilationResult<'a, ()> {
    for alias in self.aliases.values() {
      self.resolve_alias(alias)?;
    }

    Ok(())
  }

  fn resolve_alias(&self, alias: &Alias<'a>) -> CompilationResult<'a, ()> {
    let token = self.alias_tokens.get(&alias.name).unwrap();
    // Make sure the alias doesn't conflict with any recipe
    if let Some(recipe) = self.recipes.get(alias.name) {
      return Err(token.error(AliasShadowsRecipe {
        alias: alias.name,
        recipe_line: recipe.line_number,
      }));
    }

    // Make sure the target recipe exists
    if self.recipes.get(alias.target).is_none() {
      return Err(token.error(UnknownAliasTarget {
        alias: alias.name,
        target: alias.target,
      }));
    }

    Ok(())
  }
}
