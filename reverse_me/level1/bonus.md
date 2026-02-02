# Level 1 Patch

## Situation
On sait que le programme valide le mot de passe de cette façon :
```c
strcmp(input, "__stack_check")
```

Afin de le rendre valide avec n'importe quel mot de passe, on peut le modifier afin qu'il fasse le test:
```c
strcmp(input, input)
```

## Patch
1. Ouvrir le binaire
2. Trouver le strcmp
3. Trouver l'instruction mov qui place le second argument
4. Modifier l'instruction pour mov la même valeur que le premier argument
5. Sauvegarder le binaire modifié
