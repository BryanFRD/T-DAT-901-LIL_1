# Guide de démarrage

## 1. Prérequis

- **Java 21** (la version exacte est indiquée dans `.sdkmanrc`)
- **Maven**
- **IntelliJ IDEA Ultimate** (licence gratuite pour étudiants/enseignants)

## 2. Installation de SDKMAN!

```sh
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"
```

## 3. Installation de Java 21 avec SDKMAN!

Utiliser la version indiquée dans `.sdkmanrc`:

```sh
sdk env install
```
### ou 
```sh
sdk install java "version"
```

## 4. Activation de l'environnement SDKMAN!

```sh
sdk env
```
### ou 
```sh
sdk use java "version"
```

## 5. Installation de Maven

```sh
sdk install maven
```

## 6. Compilation du projet

```sh
mvn clean install
```

## 7. Configuration d'IntelliJ IDEA
- Ouvrir IntelliJ IDEA Ultimate.
- Cliquer sur "Current File" et "Edit configuration".
- Ajouter une nouvelle configuration "Spring boot".
- Sélectionner la classe principale `com.epitech.cryptoviz_backend.CryptovizBackendApplication`.
- Selectionner votre version de Java lié avec SDKMAN!.
- Activer le profil `local`
- Appliquer et OK.

## 8. Exécution de l'application
- Cliquer sur le bouton "Run" dans IntelliJ IDEA.
- L'application devrait démarrer et être accessible à l'adresse `http://localhost:8080`.

# Happy coding!