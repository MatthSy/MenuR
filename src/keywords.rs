use crate::entries::Entry;
use std::collections::{hash_map, HashMap};
use std::rc::Rc;

#[derive(Debug)]
// TODO: Mettre les entry du vec dans une Box ? (ou Rc, ou jsp...)
pub(crate) struct Keywords(hash_map::HashMap<String, Vec<Rc<Entry>>>);

impl Keywords {
    pub(crate) fn new() -> Self {
        Keywords(HashMap::default())
    }

    // TODO: fix ça pour pas clone à chaque fois mais ne plus avoir de pb de lifetime
    pub(crate) fn from(entries: Vec<Entry>) -> Self {
        let mut result = Self::new();

        for entry in entries {
            // First : add entries' names as a keyword :
            if result.0.contains_key(&entry.name) {
                // TODO: add entry when keyword already exists
            } else {
                result.0.insert(entry.name.clone(), vec![Rc::new(entry)]);
            }

            // Then add actual keywords :
            for keyword in entry.key_words {
                if result.0.contains_key(&keyword) {
                    // TODO: add entry when keyword already exists
                } else {
                    result.0.insert(entry.name.clone(), vec![Rc::new(entry)]);
                }
            }
        }
        result
    }
}

// Actuellement :
// Parcours de chaque ligne (obligatoire dans la fonction filtre ça) et je compare le nom avec la
// recherche. Plus ou moins O(n).

// Pour ajouter les keywords à la recherche :
//
// Soit : Je construit une hashmap de keywords (qui contient aussi les noms des entries) qui pointent
// vers le(s) nom(s) de l'entry correspondant. Pour la recherche, je la parcours et en construit
// une liste d'entry correpondant à la recherche. Puis je parcours les entries en les comparant à
// ceux de ma liste d'entry valides.
// Complexité : 1) Construire la Hashmap: O(m*n), m est petit normalement, 2) parcourir les
// lignes pour comparer aux keywords : O(n)
//
// Soit : Trouver un moyen d'ajouter de la data aux widgets, y mettre les keywords correspondant à
// la création de la lignes. Probleme : Unsafe  probablement
// Complexité : 1) ajouter la data : O(m*n), 2) parcourir les lignes O(n)
