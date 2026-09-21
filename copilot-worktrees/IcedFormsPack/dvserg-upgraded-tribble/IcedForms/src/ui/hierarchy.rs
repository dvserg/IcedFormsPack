// -----------------------------------------------------------------------------
// Модуль hierarhy
// Содержит реализацию построения дерева виджетов
// -----------------------------------------------------------------------------
use crate::core::*;
//use crate::ui;

/// Легковесная пара ссылок для топологического анализа без аллокаций памяти
#[derive(Debug, Clone)]
struct TreeLink {
    id: String,
    parent: String,
    visited: bool,
}

/// Главная функция-фильтр. Возвращает список ID виджетов, упорядоченный сверху-вниз (от корня к листьям).
/// Полностью исключает любые кольца и зацикливания.
///
/// `skip_subtree_id` — опциональный ID виджета. Если он передан, этот виджет
/// и вся его подветка будут полностью выброшены из анализа (идеально для поиска целей перемещения).

pub fn get_safe_hierarchy(factory: &Factory, skip_subtree_id: Option<&str>) -> Vec<String> {
    // Собираем плоский список пар из фабрики на полноценных String во владении
    let mut flat_list: Vec<TreeLink> = factory
        .blueprints_iter()
        .map(|(id, blueprint_rc)| {
            let parent = factory.get(id, PROP_PARENT).unwrap_or_default();

            TreeLink {
                id:      id.to_string(), // Клонируем &String ключа в независимую String
                parent,                  // parent уже является String из unwrap_or_default
                visited: false,
            }
        })
        .collect();

    // Если задан skip_subtree_id, каскадно маркируем всю его подветку как visited,
    // чтобы алгоритм вообще не брал их в расчет (они не попадут в безопасные цели)
    if let Some(skip_id) = skip_subtree_id {
        mark_subtree_visited(&mut flat_list, skip_id);
    }

    let mut result = Vec::with_capacity(flat_list.len());

    // Очередь для рекурсивного обхода (хранит String родителей)
    let mut queue = Vec::new();

    // ПЕРВЫЙ ПРОХОД: Ищем корни дерева (parent == "" или parent == "root")
    for link in &mut flat_list {
        if !link.visited && (link.parent.is_empty() || link.parent == "root") {
            link.visited = true;
            result.push(link.id.clone()); // Клонируем String в итоговый результат
            queue.push(link.id.clone()); // Корни становятся первыми «предками» для исследования
        }
    }

    // ЦИКЛ ОБХОДА (Топологический сито-фильтр)
    // Пока в очереди есть предки, ищем их прямых детей на следующем уровне
    let mut head = 0;
    while head < queue.len() {
        // ИСПРАВЛЕНО: Клонируем строку из очереди во владение переменной,
        // полностью освобождая вектор `queue` от любых ссылок!
        let current_parent = queue[head].clone();
        head += 1;

        for i in 0..flat_list.len() {
            // Теперь сравниваем две полноценные String напрямую без разыменования
            if !flat_list[i].visited && flat_list[i].parent == current_parent {
                flat_list[i].visited = true;
                result.push(flat_list[i].id.clone());

                // Rust теперь без проблем разрешает пушить в queue,
                // так как вектор больше никто не держит по ссылке!
                queue.push(flat_list[i].id.clone());
            }
        }
    }

    // ЛОГИРОВАНИЕ ОШИБОК: Если после обхода в flat_list остались не посещенные элементы,
    // и мы не просили их пропускать — значит, в базе данных ЕСТЬ мертвые кольца!
    for link in flat_list {
        if !link.visited {
            // Если мы пропускали поддерево, то не спамим ошибкой на элементы этого поддерева
            if let Some(skip_id) = skip_subtree_id {
                if link.id == skip_id || link.parent == skip_id {
                    continue;
                }
            }

            log::warn!(
                "get_safe_hierarchy: Обнаружено и изолировано мертвое кольцо в базе! Виджет оторван от root: widget_id: '{}' parent_id: '{}'. ",
                link.id,
                link.parent
            );
        }
    }

    result
}

/// Вспомогательная рекурсивная функция для пометки поддерева как «посещенного» (исключение из графа)
fn mark_subtree_visited(flat_list: &mut [TreeLink], subtree_id: &str) {
    // Находим сам элемент и маркируем его
    for i in 0..flat_list.len() {
        if flat_list[i].id == subtree_id && !flat_list[i].visited {
            flat_list[i].visited = true;

            // Клонируем ID для рекурсии, чтобы обойти ограничения заимствований Rust
            let next_target = flat_list[i].id.to_string();

            // Ищем всех его прямых детей и отправляем их по цепочке вниз
            for j in 0..flat_list.len() {
                if flat_list[j].parent == next_target {
                    // ИСПРАВЛЕНО: Клонируем ID ребенка в независимую переменную String,
                    // полностью снимая заимствование с массива flat_list!
                    let child_id = flat_list[j].id.clone();

                    // Теперь flat_list свободен, передаем ссылку на локальную строку
                    mark_subtree_visited(flat_list, &child_id);
                }
            }
            break;
        }
    }
}


