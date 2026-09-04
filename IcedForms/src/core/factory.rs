// -----------------------------------------------------------------------------
// Модуль factory
// Содержит реализацию фабрики для управления blueprints
// -----------------------------------------------------------------------------
use iced::{Color, border::Radius};
use indexmap::IndexMap;
use log::{info, warn};
use std::cell::RefCell;
use std::collections::{BTreeMap};
use std::rc::Rc;

use crate::core::prop_keys::*;
use crate::core::utils::*;
use crate::core::{PropertyKey, PropertyStorage};
use crate::core::{WidgetBlueprint, WidgetCreator};
use crate::core::{fnv1a_hash_64, get_prop_type_hash};



#[derive(Debug)]
pub struct Factory {
    // КАТАЛОГ СТРОКОВЫХ ТИПОВ (Динамический реестр фабрик)
    //     Содержит список всех типов виджетов, которые вообще существуют и зарегистрированы в вашей программе.
    //     Позволяет динамически регистрировать новые типы виджетов при старте, полностью избавившись
    //     от жесткого перечисления `enum WidgetType`. При вызове `.create("text")` фабрика заглядывает сюда,
    //     находит `TextCreator` по строковому ключу и просит его породить новый чертеж.
    //     Ключ (String): Текстовое имя типа виджета в нижнем регистре (например: "text", "button", "input").
    pub creators: BTreeMap<String, Rc<dyn WidgetCreator>>,

    // СКЛАД ЧЕРТЕЖЕЙ (Структурный стейт интерфейса)
    //     Содержит готовые экземпляры настроенных виджетов (их размеры, ID, плейсхолдеры).
    //     Ключ (String): Уникальный ID виджета в системе (например, "widget_1", "widget_2").
    //pub blueprints: IndexMap<String, Rc<dyn WidgetBlueprint>>,
    blueprints: IndexMap<String, Rc<dyn WidgetBlueprint>>,

    // БАЗА ДАННЫХ ПОЛЕЙ (Стейт пользовательских данных / Тексты)
    //     Содержит живые данные, которые в данный момент отображаются или введены в виджеты.
    //     Ключ (String): Тот же самый уникальный ID виджета (например, "widget_1").
    //     Значение (String): Текстовое содержимое (текст надписи, надпись на кнопке или то, что ввел пользователь в input).
    field_values: RefCell<PropertyStorage>,

    // СЧЕТЧИК ВИДЖЕТОВ (Генератор уникальности)
    //     Содержит простое число (начиная с 0), которое увеличивается на +1 при каждом создании нового виджета.
    //     Нужен чтобы гарантировать, что ни один виджет в программе не получит одинаковый ID.

    // !!! widget_counter ???
    pub field_counter: usize,

    // Режим дизайнера
    //#[serde(skip_deserializing, skip_serializing)]
    is_design_mode: bool,

    // Единый кеш рендер-стейтов, который можно использовать для нескольких типов виджетов.
    // Убираем: Удобнее сделать локальные states в блюпринтах на UnsafeCell<пропертях>
    //render_cache: RefCell<HashMap<String, RenderCacheEntry>>,
}

// -----------------------------------------------------------------------------
// СИНХРОНИЗАЦИИ ДЛЯ ICED: Помечаем нашу быструю однопоточную фабрику
// как системно безопасную для переноса (Send + Sync).
// Это заставит Iced принять её на верхнем уровне!
// -----------------------------------------------------------------------------
unsafe impl Send for Factory {}
unsafe impl Sync for Factory {}


// =============================================================================
// Реализция функций для Factory
// =============================================================================
impl Factory {
    // -------------------------------------------------------------------------
    // Полная очистка созданных виджетов и свойств
    pub fn clear_all(&mut self) {
        // Очистка чертежей виджетов
        self.blueprints.clear();

        // Очистка хранилища свойств виджетов
        self.field_values.borrow_mut().clear_all();

        // Сброс счетчика ID виджетов
        self.field_counter = 0;

        // Очистка общего кеша рендер-стейтов
        //self.render_cache.borrow_mut().clear();

        log::info!(
            "Factory: Выполнена очистка хранилища чертежей и свойств, счетчик ID сброшены в 0."
        );
    }

    // =========================================================================
    // Creators - реестр зарегистрированных конструкторов чертежей
    // =========================================================================
    /// Регистрация базовых виджетов вручную (если не используется авторегистрация из инвентаря)
    pub fn creator_register_type(&mut self, name: &str, creator: std::rc::Rc<dyn WidgetCreator>) {
        self.creators.insert(name.to_string(), creator);

        log::info!("Регистрация '{name}'");
    }

    // =========================================================================
    // Blueprints - склад чертежей созданных виджетов
    // =========================================================================
    // Создание экземпляра виджета
    // Возвращает ID созданного виджета
    pub fn create_blueprint(&mut self, w_type: &str) -> String {
        self.field_counter += 1;
        let unique_id = format!("widget_{}", self.field_counter);

        // Ищем создателя в карте по строковому имени
        if let Some(creator) = self.creators.get(w_type) {
            // Создаем чертеж виджета через
            let bp = creator.create_blueprint(unique_id.clone());

            bp.from_vtab(self);

            // *** Обновляем internal property
            // Тестовая фича для проверки разницы быстродействия чтения VTABLE и локальных UnsafeCell<пропертей> в блюпринтах
            // Пока VTABLE круче по скорости (возможно из-за оптимизаций компилятора)
            // На постоянку UsafeCell<проперти> остаются в интерактивных блюпринтах типа text_edit (markdown, combobox, ..)
            bp.refresh_internal_props(self);

            // Сохраняем чертеж в IndexMap
            //self.blueprints.insert(unique_id.clone(), bp.clone());        // убрать, добавил инкапсуляцию внутренних полей (ниже)
            self.insert_blueprint(&unique_id, bp);

            info!("Добавлен: Виджет '{}' с ID '{}'.", w_type, unique_id);
        } else {
            warn!(
                "Предупреждение: Тип виджета '{}' не зарегистрирован!",
                w_type
            );
        }

        unique_id
    }

    // -------------------------------------------------------------------------
    // Вставляет чертеж в IndexMap
    pub fn insert_blueprint(&mut self, unique_id: &String, blueprint: Rc<dyn WidgetBlueprint>) {
        self.blueprints.insert(unique_id.clone(), blueprint.clone());
    }

    // -------------------------------------------------------------------------
    // Очистка чертежей виджетов
    pub fn clear_blueprints(&mut self) {
        self.blueprints.clear();
    }

    // -------------------------------------------------------------------------
    // Функция 'get_blueprint' возвращает настроенный блупринт виджета
    // по его уникальному ID. Если виджет с таким идентификатором существует
    // на складе чертежей, метод вернет Some(Rc<dyn WidgetBlueprint>), иначе — None.
    // -------------------------------------------------------------------------

    // Автономная долгоживущия ссылка на блюпринт с увеличением счетчика ссылок
    // Функция возвращает нового независимого совладельца блюпринта
    pub fn get_blueprint(&self, id: String) -> Option<Rc<dyn WidgetBlueprint>> {
        // Ищем в IndexMap по строковой ссылке и клонируем Rc указатель
        self.blueprints.get(&id).cloned()
    }

    // -------------------------------------------------------------------------
    // Быстрая прямая временная ссылка на блюпринт без счетчика ссылок
    // Использовать по месту здесь и сейчас
    pub fn get_blueprint_rc<'a>(&'a self, id: String) -> Option<&'a dyn WidgetBlueprint> {
        //self.blueprints.get(id).map(|rc| rc.as_ref())
        self.blueprints.get(&id).map(|rc| rc.as_ref())
    }

    // -------------------------------------------------------------------------
    // Получаем мутабельную ссылку на блюпринт
    // Из-за Rc не можем получить мутабельную ссылку напрямую, поэтому прибегаем к unsafe
    //pub fn get_blueprint_mut<'a>(&'a mut self, id: String) -> Option<&'a mut dyn WidgetBlueprint> {
    // Получаем ИЗМЕНЯЕМУЮ ссылку на сам умный указатель Rc внутри карты
    //    let rc_mut = self.blueprints.get_mut(&id)?;

    // Используем официальный метод Rc::get_mut.
    // Поскольку в handle_widget_action этот виджет временно не рендерится,
    // счетчик ссылок равен 1, и метод честно вернет Some(&mut dyn WidgetBlueprint)
    //    Rc::get_mut(rc_mut)
    //}

    // -------------------------------------------------------------------------
    /// Находит все чертежи виджетов, у которых свойство "parent"
    /// в базе данных field_values совпадает с заданным parent_id.
    pub fn get_blueprints_by_parent<'a>(
        &'a self,
        parent_id: &str,
    ) -> Vec<&'a Rc<dyn WidgetBlueprint>> {
        self.blueprints
            .iter()
            .filter(|(id, _blueprint_arc)| {
                // Читаем родителя текущего виджета из PropertyStorage по хэшу константы!
                // Явно указываем String слева, чтобы запустить автовывод типа get()
                let current_parent: String = self.get(id, PROP_PARENT).unwrap_or_default();

                // Проверка на совпадение родителя, или проверка на принадлежость элемента корню
                current_parent == parent_id || {
                    // Проверка корня для parent_id: "" или "root"
                    let is_root_search = parent_id.is_empty() || parent_id == "root";

                    // Проверка корня для current_parent: "" или "root"
                    let is_widget_at_root = current_parent.is_empty() || current_parent == "root";

                    // Возвращаем true, если оба маркера указывают на корень
                    is_root_search && is_widget_at_root
                }
            })
            // Оставляем только ссылку на Arc, отбрасывая строковый ID ключа IndexMap
            .map(|(_id, blueprint_arc)| blueprint_arc)
            .collect() // Собираем отфильтрованные ссылки на Arc в вектор
    }

    // -------------------------------------------------------------------------
    /// Возвращает вектор, содержащий все идентификаторы (ID) виджетов.
    pub fn get_blueprint_keys(&self) -> Vec<String> {
        self.blueprints.keys().cloned().collect()
    }

    // -------------------------------------------------------------------------
    // Устанавливает между блупринтами иерархическую связь Родитель-Ребенок в VTable
    // с защитой от циклической рекурсии.
    // Метод принимает &mut self, так как мутирует свойства виджетов через индексы и метод self.set()
    pub fn set_blueprint_parent(&mut self, child_id: &str, parent_id: &str) {
        // Импортируем наш единый системный токен родителя из ядра базы данных свойств
        let prop_parent_token = PROP_PARENT;

        // Проверяем, существует ли ребенок в фабрике
        if !self.blueprints.contains_key(child_id) {
            log::warn!(
                "set_blueprint_parent: Невозможно задать родителя — ребенок '{}' не найден.",
                child_id
            );
            return;
        }

        // Нормализуем строку нового родителя
        let clean_parent = parent_id.trim().to_lowercase();
        let target_parent_id = if child_id == clean_parent || clean_parent == "root" {
            "".to_string() // Сброс в корень холста
        } else {
            clean_parent
        };

        // Если переносим в корень холста — предварительно разрешаем вставку
        let mut can_assign = target_parent_id.is_empty();

        if !target_parent_id.is_empty() {
            // 3. ЗАЩИТА ОТ РЕКУРСИИ (while-цикл)
            let mut current_check_id = target_parent_id.clone();
            let mut is_loop_detected = false;

            while !current_check_id.is_empty() {
                if current_check_id == child_id {
                    is_loop_detected = true;
                    break;
                }

                // Читаем родителя проверяемого узла по нашему центральному токену ядра
                current_check_id = self
                    .get::<String>(&current_check_id, prop_parent_token)
                    .unwrap_or_default();
            }

            if is_loop_detected {
                log::error!(
                    "set_blueprint_parent: Обнаружено зацикливание! Изменение 'parent' для '{}' заблокировано.",
                    child_id
                );
                return; // Полностью прерываем транзакцию
            }

            // Проверяем вместимость контейнера у легитимного родителя
            if let Some(parent_blueprint) = self.blueprints.get(&target_parent_id) {

                if parent_blueprint.can_accept_child(self) {
                    can_assign = true;
                } else {
                    log::warn!(
                        "set_blueprint_parent: Виджет '{}' не может принять ребенка. Вставка заблокирована.",
                        target_parent_id
                    );
                }
            } else {
                log::error!(
                    "set_blueprint_parent: Новый родитель '{}' не существует на холсте. Вставка заблокирована.",
                    target_parent_id
                );
            }
        }

        // Записываем связь ТОЛЬКО если все иерархические проверки пройдены на 100%
        if can_assign {
            // Пишем строку нового родителя в фабрику по нашему токену
            self.set(child_id, prop_parent_token, target_parent_id.clone());

        } else {
            log::warn!(
                "set_blueprint_parent: Транзакция отклонена: виджет '{}' остался на прежнем месте.",
                child_id
            );
        }
    }

    // -------------------------------------------------------------------------
    // Итератор пар (Ключ, Значение) для виджетов
    // Пример использования:
    //      for (name, blueprint_rc) in registry.blueprints_iter() {
    //          // name имеет тип: &String
    //          // blueprint_rc имеет тип: &Rc<dyn WidgetBlueprint>
    //          // let id = blueprint_rc.get_id();
    //          // или
    //          // let id_explicit = (**blueprint_rc).get_id();
    //      }
    pub fn blueprints_iter(&self) -> indexmap::map::Iter<'_, String, Rc<dyn WidgetBlueprint>> {
        self.blueprints.iter()
    }

    // -------------------------------------------------------------------------
    // Выполняет подмену блупринта на Dummy в приватной коллекции,
    // дает уникальный мутабельный доступ к нему и возвращает всё на место.
    // Пример:
    //  // Вызываем инкапсулированный метод подмены на фабрике
    //  let task_opt = self.factory.with_blueprint_mut(widget_id, |bp_mut| {
    //      Внутри замыкания bp_mut имеет чистый тип &mut dyn WidgetBlueprint
    //      Вызываем ваш оригинальный метод handle_event с &mut self!
    //      bp_mut.handle_event(&widget_action_cl, self)
    //  });
    //
    pub fn with_blueprint_mut<F, R>(&mut self, widget_id: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut dyn WidgetBlueprint) -> R,
    {
        // Находим мутабельный слот указателя в приватном IndexMap по ID
        if let Some(rc_slot) = self.blueprints.get_mut(widget_id) {
            
            // Получаем очередной системный id для виджета
            let new_id = self.field_counter;
            self.field_counter += 1;

            // Вынимаем оригинальный Rc, вставляя заглушку
            let mut bp_rc = std::mem::replace(
                rc_slot, 
                Rc::new(crate::core::DummyBlueprint::new(format!("dummy_{}", new_id)))
            );

            log::info!("Factory: Оригинальный блупринт успешно извлечен на месте.");

            // Получаем &mut dyn WidgetBlueprint через Rc::get_mut
            let bp_mut = Rc::get_mut(&mut bp_rc).expect(
                "КРИТИЧЕСКАЯ ОШИБКА: Указатель Rc не уникален при подмене на месте!",
            );

            // -----------------------------------------------------------------
            // Выполняем внешнюю логику над мутабельным блупринтом
            // -----------------------------------------------------------------
            let result = f(bp_mut);

            // ВОЗВРАЩАЕМ ВСЁ НА СВОИ МЕСТА:
            if let Some(rc_slot_back) = self.blueprints.get_mut(widget_id) {
                let _ = std::mem::replace(rc_slot_back, bp_rc);
            }

            Some(result)
        } else {
            None
        }
    }

    // -------------------------------------------------------------------------
    // Реализация мутабельности блупринт через извлечение объекта из IndexMap 
    // с подменой на DummyBlueprint для сохранения места и сортировки объектва 
    // в массиве
    // -------------------------------------------------------------------------
    // Шаг А: Извлекает оригинальный блупринт, временно оставляя вместо него Dummy.
    // Порядок элементов в IndexMap сохраняется.
    pub fn take_blueprint_for_event(&mut self, widget_id: &str) -> Option<Rc<dyn WidgetBlueprint>> {
        if let Some(rc_slot) = self.blueprints.get_mut(widget_id) {
            // Заменяем оригинальный Rc на пустышку DummyBlueprint и возвращаем оригинал
            let original_bp = std::mem::replace(
                rc_slot, 
                Rc::new(crate::core::DummyBlueprint::new("dummy".to_string()))
            );
            Some(original_bp)
        } else {
            None
        }
    }

    // Шаг Б: Возвращает измененный блупринт обратно на свое место.
    pub fn put_blueprint_back(&mut self, widget_id: &str, original_bp: Rc<dyn WidgetBlueprint>) {
        if let Some(rc_slot) = self.blueprints.get_mut(widget_id) {
            // Возвращаем оригинал, затирая временный Dummy
            let _ = std::mem::replace(rc_slot, original_bp);
        }
    }
    //
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Возвращает вектор идентификаторов (ID) всех виджетов, которые являются
    // прямыми потомками для указанного `parent_id`.
    pub fn get_blueprint_keys_by_parent(&self, parent_id: &str) -> Vec<String> {
        self.blueprints
            .keys() // Бежим строго по ключам, не трогая Rc внутри кучи
            .filter(|id| {
                // Вытаскиваем родителя текущего виджета из VTable
                let current_parent: String = self.get(id, PROP_PARENT).unwrap_or_default();

                // Проверка на точное совпадение родителя
                current_parent == parent_id || {
                    // Ситуация А: Ищем элементы корня
                    let is_root_search = parent_id.is_empty() || parent_id == "root";

                    // Ситуация Б: Текущий виджет лежит в корне холста
                    let is_widget_at_root = current_parent.is_empty() || current_parent == "root";

                    // Элемент подходит, если мы ищем корень и виджет реально в корне
                    is_root_search && is_widget_at_root
                }
            })
            .cloned() // Клонируем &String ключа в независимую, долгоживущую String
            .collect() // Собираем отфильтрованные ID в итоговый Vec<String>
    }

    // -------------------------------------------------------------------------
    // Реактивное добавление виджета с автоматическим поиском ближайшего родительского контейнера.
    // Метод принимает &mut self, так как вызывает self.create() и мутирует дерево blueprints.
    pub fn add_widget(&mut self, widget_type: &str, selected_id: Option<&str>) -> String {
        // Создаем блупринт виджета в памяти фабрики (возвращает новый ID, например "widget_1")
        let new_id = self.create_blueprint(widget_type);

        // Иерархический поиск правильной цели для вставки с использованием can_accept_child
        let target_parent_id = match selected_id {
            Some(active_id) => {
                let mut current_search_id = active_id.to_string();
                let mut found_container = "root".to_string();

                while !current_search_id.is_empty() && current_search_id != "root" {
                    if let Some(bp) = self.blueprints.get(&current_search_id) {
                        // Магия архитектуры: спрашиваем у блупринта, готов ли он принять ребенка
                        if bp.can_accept_child(self) {
                            found_container = current_search_id;
                            break; // Ближайший легальный контейнер найден, останавливаем поиск вверх
                        }
                    }

                    // Чтение ID родителя из VTable через стандартный метод self.get()
                    current_search_id = self
                        .get::<String>(&current_search_id, PROP_PARENT)
                        .unwrap_or_default();
                }

                if found_container == "root" {
                    None
                } else {
                    Some(found_container)
                }
            }
            None => None, // Если на холсте ничего не выбрано — вставляем элемент в корень холста
        };

        // Финализируем привязку родителя в VTable-базе данных
        if let Some(real_parent_id) = target_parent_id {
            // Запускаем логику связывания индексов и записи родителя
            self.set_blueprint_parent(&new_id, &real_parent_id);

            log::info!(
                "add_widget: Виджет '{}' успешно добавлен в контейнер '{}'",
                new_id,
                real_parent_id
            );
        } else {
            // Записываем пустую строку в родительский ключ, если контейнеров выше нет.
            self.set(&new_id, PROP_PARENT, "".to_string());
            log::info!("add_widget: Виджет '{}' создан в корне холста.", new_id);
        }

        new_id // Возвращаем сгенерированный ID свежесозданного элемента наружу
    }

    // Возвращает вектор детских виджетов по parent_id
    pub fn get_children_ids_by_parent(&self, parent_id: &str) -> Vec<String> {
        self.blueprints
            .iter()
            .filter(|(id, _)| {
                let current_parent: String = self.get(id, PROP_PARENT).unwrap_or_default();
                current_parent == parent_id || {
                    let is_root_search = parent_id.is_empty() || parent_id == "root";
                    let is_widget_at_root = current_parent.is_empty() || current_parent == "root";
                    is_root_search && is_widget_at_root
                }
            })
            .map(|(id, _)| id.clone()) // Клонируем только строковый ID, отпуская ссылки ОЗУ
            .collect()
    }

    // Каскадно и рекурсивно удаляет виджет, всех его потомков и их свойства из фабрики
    pub fn delete_widget(&mut self, widget_id: &str) {
        // Шаг 1: Извлекаем плоский изолированный список ID детей, не блокируя self!
        let children_to_delete = self.get_children_ids_by_parent(widget_id);

        // Запускаем каскадный рекурсивный спуск по изолированным строкам на стеке
        for child_id in children_to_delete {
            log::trace!(
                "delete_widget: Каскадный спуск -> Удаление потомка '{}'",
                child_id
            );
            self.delete_widget(&child_id); // Рекурсия работает идеально!
        }

        // Шаг 2: Ветка очищена снизу вверх, теперь безопасно вырезаем сам узел
        self.blueprints.shift_remove(widget_id);

        // Рассчитываем рантайм-хэш удаляемого виджета
        let widget_hash = crate::core::fnv1a_hash_64(widget_id);

        // Открываем изменяемый замок RefCell на запись без какого-либо оверхеда локов ОС
        let mut storage_guard = self.field_values.borrow_mut();

        // Удаляем виджет и его свойства из хранилища
        storage_guard.remove_widget_by_hash(widget_hash);

        log::info!(
            "delete_widget: Успешно выжжен виджет '{}' и вся его VTable-иерархия.",
            widget_id
        );
    }

    // Перемещает виджет внутри родителя
    pub fn move_widget_in_map(&mut self, widget_id: &str, direction: i32) {
        //use crate::form::factory::core::property::PROP_PARENT;

        // Получить предка виджета.
        // Явно указываем String слева, чтобы запустить автовывод типа get() по хэшу PROP_PARENT
        let parent_id: String = self.get(widget_id, PROP_PARENT).unwrap_or_default();

        // Собираем упорядоченный список ВСЕХ детей этого же родителя в том порядке,
        // в котором они сейчас физически лежат в IndexMap.
        let mut local_children = Vec::new();
        for id in self.blueprints.keys() {
            // Читаем родителя текущего обходимого виджета по хэшу константы
            let current_parent: String = self.get(id, PROP_PARENT).unwrap_or_default();

            if current_parent == parent_id {
                local_children.push(id.clone()); // Клонируем строку, освобождая self
            }
        }

        // Находим позицию текущего виджета внутри этого локального среза детей
        if let Some(current_local_pos) = local_children.iter().position(|id| id == widget_id) {
            let target_local_pos = current_local_pos as i32 + direction;

            // Проверяем, что цель не выходит за границы локального массива детей
            if target_local_pos >= 0 && target_local_pos < local_children.len() as i32 {
                let target_local_pos = target_local_pos as usize;

                // Достаем ID соседа, с которым нужно поменяться местами
                let sibling_id = &local_children[target_local_pos];

                // 4. Переводим локальные позиции в ГЛОБАЛЬНЫЕ индексы физической карты IndexMap
                let global_current_idx = self.blueprints.get_index_of(widget_id);
                let global_sibling_idx = self.blueprints.get_index_of(sibling_id);

                if let (Some(g_curr), Some(g_sibl)) = (global_current_idx, global_sibling_idx) {
                    // Перестановка элементов в упорядоченной IndexMap
                    self.blueprints.swap_indices(g_curr, g_sibl);

                    println!(
                        "🔄 Локальный своп: '{widget_id}' поменялся местами с '{sibling_id}' внутри родителя '{parent_id}'"
                    );
                }
            }
        }
    }


    // =========================================================================
    // Properties - VTable свойств виджетов
    // =========================================================================

    // Возвращает динамическое заимствование на чтение или запись для всего 
    // хранилища свойств. Метод возвращает умный указатель `Ref`('RefMut'), 
    // который автоматически закроет замок заимствования, как только 
    // переменная выйдет из области видимости.    
    pub fn get_field_values(&self) -> std::cell::Ref<'_, PropertyStorage> {
        self.field_values.borrow()
    }
    pub fn get_field_values_mut(&self) -> std::cell::RefMut<'_, PropertyStorage> {
        self.field_values.borrow_mut()
    }
    
    // -------------------------------------------------------------------------
    /// Основная пара на чтение/запись в хранилище. Все другие обращения идут через нее.
    // -------------------------------------------------------------------------
    /// УЛЬТРАБЫСТРЫЙ Getter по чистым u64 хэшам (для рантайма Iced).
    /// Тип T вычисляется АВТОМАТИЧЕСКИ на основе переменной слева!
    pub fn get_by_hash<T: 'static + Clone>(&self, widget_id: &str, prop_hash: u64) -> Option<T> {
        // Тоже вроде работает, но похожий код с сеттером паникует на некоторых моментах
        //self.field_values
        //    .borrow()
        //    .get_by_hash::<T>(widget_id, prop_hash)

        // Делаем Getter с разрывом заимствования RefCell для устранения блокировок
        let result = {
            let storage = self.field_values.borrow();

            if let Some(val) = storage.get_by_hash::<T>(widget_id, prop_hash) {
                Some(val.clone())
            } else {
                None
            }
        }; // Блок закончился, storage уничтожен, а замок RefCell полностью снят!

        // Функция возвращает результат, когда фабрика уже полностью свободна
        result
    }

    /// УЛЬТРАБЫСТРЫЙ Setter по чистым u64 хэшам
    pub fn set_by_hash<T: 'static>(&self, widget_id: &str, prop_hash: u64, value: T) {
        // Вызывает панику на экспорте и в др. местах при использовани get_or_set
        //self.field_values
        //    .borrow_mut()
        //    .set_by_hash::<T>(widget_id, prop_hash, value);

        let mut should_refresh = false;

        // Проверяем не заблокирована ли мутабельная ссылка
        match self.field_values.try_borrow_mut() {
            Ok(mut storage) => {
                storage.set_by_hash::<T>(widget_id, prop_hash, value);
                should_refresh = true;
            }
            Err(_) => {
                log::error!(
                    "[Критическая ошибка] Не удалось записать свойство '{}:{}'! Мутабельная ссылка RefCell заблокирована.",
                    widget_id,
                    prop_hash
                );
            }
        }

        // Выносим обновление блюпринта за прежеды Lock зоны
        if should_refresh {
            if let Some(bp) = self.get_blueprint_rc(widget_id.to_string()) {
                // Теперь parse_props -> get_by_hash вызовет .borrow() без конфликтов и паники!
                bp.refresh_internal_props(self);
            }
        }
    }

    // -------------------------------------------------------------------------
    // Далее используем только две расположенные выше функции get_by_hash/set_by_hash
    // -------------------------------------------------------------------------
    // Универсальный Getter property по строковому ID виджета.
    //
    pub fn get<T: 'static + Clone>(&self, widget_id: &str, key: PropertyKey) -> Option<T> {
        self.get_by_hash::<T>(widget_id, key.hash)
    }

    // Универсальный Setter property по строковому ID виджета.
    pub fn set<T: 'static>(&self, widget_id: &str, key: PropertyKey, value: T) {
        self.set_by_hash::<T>(widget_id, key.hash, value);
    }

    // Универсальный Getter property по строковому ID виджета с дефолтом.
    // Если значение в VTable не нашлось, возвращаем дефолт
    // Пример: 
    //     let width: Length = factory.get_or_def(&widget_id, PROP_WIDTH, Length::Fill);
    //
    pub fn get_or_def<T: 'static + Clone>(&self, widget_id: &str, key: PropertyKey, def: T) -> T {
        // Пытаемся прочитать значение. Замок на чтение откроется и ТУТ ЖЕ ЗАКРОЕТСЯ
        let existing_value = {
            self.get_by_hash::<T>(widget_id, key.hash) 
            // FIX: ! borrow высвобождается !            
        };

        // Проверяем результат
        match existing_value {
            Some(value) => value, // Если значение нашлось — просто возвращаем его наружу!
            None => {
                def // Если значения нет — возвращаем def
            }
        }
    }

    // Универсальный Getter/Setter property по строковому ID виджета.
    // Если значения нет в базе — записывает в VTable дефолтное и возвращает его.
    // Пример: 
    //     let width: Length = factory.get_or_set(&widget_id, PROP_WIDTH, Length::Fill);
    //
    pub fn get_or_set<T: 'static + Clone>(&self, widget_id: &str, key: PropertyKey, def: T) -> T {
        // Пытаемся прочитать значение. Замок на чтение откроется и ТУТ ЖЕ ЗАКРОЕТСЯ
        let existing_value = {
            self.get_by_hash::<T>(widget_id, key.hash) 
        };

        // Проверяем результат
        match existing_value {
            Some(value) => value, // Если значение нашлось — просто возвращаем его наружу!
            None => {
                // Если значения нет — замок на чтение уже ГАРАНТИРОВАННО закрыт.
                // Мы можем абсолютно легально и безопасно вызвать мутабельный .set()!
                self.set_by_hash::<T>(widget_id, key.hash, def.clone());
                def
            }
        }
    }

    // УЛЬТРАБЫСТРЫЙ Getter/Setter по чистым u64 хэшам
    pub fn get_or_set_by_hash<T: 'static + Clone>(
        &self,
        widget_id: &str,
        prop_hash: u64,
        def: T,
    ) -> T {
        let existing_value = { self.get_by_hash::<T>(widget_id, prop_hash) };

        match existing_value {
            Some(value) => value,
            None => {
                self.set_by_hash::<T>(widget_id, prop_hash, def.clone());
                def
            }
        }
    }

    // -------------------------------------------------------------------------
    // Возвращает данные в текстовом виде
    pub fn get_as_string<'a>(
        &'a self,
        widget_id: &str,
        prop_key: PropertyKey,
        default_value: &str,
    ) -> String {
        let t_hash = get_prop_type_hash(prop_key);
        let key = prop_key.hash;

        //-----------------------------------------------------------------
        // ЛОКАЛЬНЫЕ КОНСТАНТЫ КОМПИЛЯЦИИ:
        // Считаются 1 раз при сборке файла!
        // Они совпадают с тем, что генерирует stringify!() в макросе!
        // Поэтому все константы создаем голым именем соблюдая регистр
        //-----------------------------------------------------------------
        // !!! Сделай глобальные константы типов в одном месте !!!

        const TYPE_STRING:      u64 = fnv1a_hash_64("String");
        const TYPE_USIZE:       u64 = fnv1a_hash_64("usize");
        const TYPE_FLOAT:       u64 = fnv1a_hash_64("f32");
        const TYPE_BOOL:        u64 = fnv1a_hash_64("bool");
        const TYPE_LENGTH:      u64 = fnv1a_hash_64("Length");
        const TYPE_PADDING:     u64 = fnv1a_hash_64("Padding");
        const TYPE_PIXELS:      u64 = fnv1a_hash_64("Pixels");
        const TYPE_RADIUS:      u64 = fnv1a_hash_64("Radius");
        const TYPE_COLOR:       u64 = fnv1a_hash_64("Color");
        const TYPE_FONT:        u64 = fnv1a_hash_64("Font");
        const TYPE_ALIGN_ITEMS: u64 = fnv1a_hash_64("Alignment");
        const TYPE_HORIZONTAL:  u64 = fnv1a_hash_64("Horizontal");
        const TYPE_VERTICAL:    u64 = fnv1a_hash_64("Vertical");

        // В зависимости от типа извлекаем данные и превращаем их в строку
        let mut type_name = "";
        match t_hash {
            TYPE_USIZE => {
                if let Some(val) = self.get_by_hash::<usize>(widget_id, key) {
                    return val.to_string();
                }
                type_name = "usize";
            }
            TYPE_FLOAT => {
                if let Some(val) = self.get_by_hash::<f32>(widget_id, key) {
                    let val_fmt = format!("{:.1}", val);
                    return val_fmt;
                }
                type_name = "Float";
            }
            TYPE_BOOL => {
                if let Some(val) = self.get_by_hash::<bool>(widget_id, key) {
                    return val.to_string();
                }
                type_name = "Bool";
            }
            TYPE_LENGTH => {
                if let Some(val) = self.get_by_hash::<iced::Length>(widget_id, key) {
                    return cast_length_2_string(val);
                }
                type_name = "Length";
            }
            TYPE_PADDING => {
                if let Some(val) = self.get_by_hash::<iced::Padding>(widget_id, key) {
                    return cast_padding_2_string(val);
                }
                type_name = "Padding";
            }
            TYPE_PIXELS => {
                if let Some(val) = self.get_by_hash::<iced::Pixels>(widget_id, key) {
                    return cast_pixels_2_string(val);
                }
                type_name = "Pixels";
            }
            TYPE_RADIUS => {
                if let Some(val) = self.get_by_hash::<Radius>(widget_id, key) {
                    return cast_radius_2_string(val);
                }
                type_name = "Radius";
            }
            TYPE_COLOR => {
                if let Some(val) = self.get_by_hash::<Color>(widget_id, key) {
                    return cast_color_2_hex(val);
                }
                type_name = "Color";
            }
            TYPE_FONT => {
                return "Font".to_string();
                //type_name = "Font";
            }
            TYPE_ALIGN_ITEMS => {
                if let Some(val) = self.get_by_hash::<iced::Alignment>(widget_id, key) {
                    return cast_align_items_2_string(val);
                }
                type_name = "AlignItems";
            }
            TYPE_HORIZONTAL => {
                if let Some(val) = self.get_by_hash::<iced::alignment::Horizontal>(widget_id, key) {
                    return cast_align_x_2_string(val);
                }
                type_name = "Horizontal";
            }
            TYPE_VERTICAL => {
                if let Some(val) = self.get_by_hash::<iced::alignment::Vertical>(widget_id, key) {
                    return cast_align_y_2_string(val);
                }
                type_name = "Vertical";
            }
            TYPE_STRING | _ => {
                if let Some(val) = self.get_by_hash::<String>(widget_id, key) {
                    return val;
                }
                type_name = "String";
            }
            _ => {
                log::warn!(
                    r#"Factory::get_as_string: Не найден зарегистрированный тип для '{}:{}'
                    Будет возвращено значение по умолчанию '{}'."#,
                    widget_id,
                    prop_key.name,
                    default_value.to_string()
                );
            }
        }

        // Не добавлен тип или не соответствуют тип в макросе и в парсере
        log::warn!(
            "Factory::get_as_string: Не удалось преобразовать <{}> '{}:{}' в строку. Проверьте соответствие типа PropetyKey.",
            type_name,
            widget_id,
            prop_key.name
        );

        // Если свойства нет в базе или тип неизвестен — возвращаем дефолт
        default_value.to_string()
    }

    // =========================================================================
    // Управление состоянием 'Design mode'
    // =========================================================================

    // Получить статус
    pub fn is_design_mode(&self) -> bool {
        return self.is_design_mode;
    }

    // Установить статус
    pub fn set_design_mode(&mut self, mode: bool) {
        self.is_design_mode = mode;
    }

    // Переключить статус
    pub fn toggle_design_mode(&mut self) {
        self.is_design_mode = !self.is_design_mode;
    }



    pub fn get_field_counter(&self) -> usize {
        self.field_counter
    }
}

// -----------------------------------------------------------------------------
//
//use crate::core::AutoRegisteredWidget;

impl Default for Factory {
    fn default() -> Self {
        let mut factory = Self {
            blueprints:     IndexMap::new(),
            field_values:   RefCell::new(crate::core::PropertyStorage::new()),
            field_counter:  0,
            creators:       BTreeMap::new(),
            // Начальное значение переменной устанавливаем в design_mode
            is_design_mode: true,
            //render_cache:   RefCell::new(HashMap::new()),
        };

        // --- ИНТЕГРАЦИЯ АВТОМАТИЧЕСКОЙ РЕГИСТРАЦИИ INVENTORY ---
        // Сканируем все файлы проекта, где прописан макрос `inventory::submit!`
        for widget in inventory::iter::<crate::core::AutoRegisteredWidget> {
            let name_string = widget.name.to_string();

            // Вызываем функцию-конструктор, получая плоский Box от линкера
            let boxed_creator = (widget.constructor)();

            // Безопасно переупаковываем Box в наш легкий однопоточный Rc!
            // Rust автоматически сотрет маркеры Send+Sync, превратив тип в Rc<dyn WidgetCreator>
            let creator_rc: Rc<dyn crate::core::WidgetCreator> =
                Rc::from(boxed_creator as Box<dyn crate::core::WidgetCreator>);

            // Регистрируем конструктор в BTreeMap creators (работает на быстрых Rc!)
            factory.creators.insert(name_string.clone(), creator_rc);
        }

        log::trace!("Default for Factory: Статус: \n{:#?}", factory);

        factory
    }
}
