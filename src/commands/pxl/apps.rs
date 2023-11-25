use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn apps(ctx: Context<'_>) -> Result<(), Error> {
    let base = "# Aplicaciones para Pixel-Art
*¡Estas son algunas aplicaciones que recomendamos para hacer Pixel Art! :D*

* [👑] Aseprite            
* Libresprite              
* Resprite               
* Pixel Studio                
* Pixilart                 
* Piskel                                             
* Pixelorama                            
* Lospec Pixel Editor
### ¿Te interesó alguna?
*¡Puedes dar clic en los botones de abajo para ir a verlas!* ⬇️";

    let aseprite = r###"
# [👑] Aseprite            

 **Tipo:** De 
 **Plataforma:** para Windows, Mac y Linux.
 **link:** https://www.aseprite.org/download/
 
 "###;

    let resprite = r###"
# Resprite            

 **Tipo:** ¡Gratis!
 **Plataformas:** IPhone - Android.
 **link:** https://resprite.fengeon.com/
  
  "###;

    let pixel_studio = r###"
# Pixel Studio

 **Tipo:** ¡Gratis!
 **Multiplataformas:** Móvil, Windows, Mac y iPhone/iPad.
 **link:** https://cutt.ly/IwIK7Lat

   "###;

    let pixilart = r###"
# Pixilart

 **Tipo:** ¡Gratis!
 **Multiplataformas:** Navegador, en Android y en iPhone/iPad
 **link:** https://www.pixilart.com

    "###;

    let piskel = r###"
# Piskel

 **Tipo:** ¡Gratis!
 **Multiplataformas:** Navegador, Windows, Mac y Linux.
 **link:** https://piskelapp.com/download

    "###;

    let pixelrama = r###"
# Pixelorama

 **Tipo:** ¡Gratis!
 **Multiplataformas:** Windows, Mac y Linux.
 **link:** https://cutt.ly/iwILqf8h
 
  "###;

    let lospec = r###"
# Lospec

 **Tipo:** ¡Gratis!
 **Plataforma:** Navegador.
 **link:** https://lospec.com/pixel-editor/

    "###;

    let pages = &[
        base,
        aseprite,
        resprite,
        pixel_studio,
        pixilart,
        piskel,
        pixelrama,
        lospec,
    ];

    poise::samples::paginate(ctx, pages).await?;

    Ok(())
}
