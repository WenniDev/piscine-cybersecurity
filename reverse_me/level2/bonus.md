# Level 2 Patch

## Situation
Le programme valide le mot de passe en lui faisant passer une serie de testes et execute la fonction `no()` qui exit en cas d'erreurs.

```c
void no() {
    puts("Nope.");
    exit(1);
}
```

Pour rendre que le programme execute la fonction `ok()` pour n'importe quel mot de passe, on peut remplacer par des NOP toutes les instructions entre `no()` et `ok()` car ces deux fonctions sont située à la suite, il y a la fonction `xd()` qu'on nulifie totalement.

```c
void no() {
//     puts("Nope.");
//     exit(1);
// }

// int xd() {
//     puts("Iii sapores crescit rei habetur disputo. An ab istud mo prius tanta error "
//     "debet. Firma foret tes mea age capax sumne. Ex ex ipsas actum culpa neque ab saepe. "
//     "Existenti et principia co immittere probandam imaginari re mo. Quapropter "
//     "industriam ibi cui dissimilem cucurbitas progressus perciperem. Essendi ratione si "
//     "habetur gi ignotas cognitu nusquam et.Sumpta vel uti ob");
//     return puts("Author gi ex si im fallat istius. Refutent supposui qua sim "
//     "nihilque. Me ob omni ideo gnum casu. Gi supersunt colligere inhaereat me sapientia "
//     "is delaberer. Rom facillimam rem expe");
// }

// int ok() {
    return puts("Good job.");
}```

## Patch
1. Ouvrir le binaire
2. Trouves les fonctions `ok()`, `xd()` et `no()`
3. Remplacer par des NOP les instructions de `11221` à `112a0`
5. Sauvegarder le binaire modifié

A partir de maintenant, la fonction `no()` executeras le contenu de la fonction `ok()`
