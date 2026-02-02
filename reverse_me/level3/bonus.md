# Level 2 Patch

## Situation
Le programme valide le mot de passe en lui faisant passer une serie de testes et execute la fonction `___syscall_malloc()` qui exit en cas d'erreurs.

```c
void ___syscall_malloc() {
    puts("Nope.");
    exit(1);
}
```

Pour rendre que le programme execute la fonction `____syscall_malloc()` pour n'importe quel mot de passe, on peut remplacer par des NOP toutes les instructions entre `___syscall_malloc()` et `____syscall_malloc()` car ces deux fonctions sont située à la suite.

```c
void ___syscall_malloc() {
//     puts("Nope.");
//     exit(1);
// }

// int ____syscall_malloc() {
    return puts("Good job.");
}
```

## Patch
1. Ouvrir le binaire
2. Trouves les fonctions `___syscall_malloc()` et `____syscall_malloc()`
3. Remplacer par des NOP les instructions de `4012e0` à `401300`
5. Sauvegarder le binaire modifié

A partir de maintenant, la fonction `___syscall_malloc()` executeras le contenu de la fonction `____syscall_malloc()`
