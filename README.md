# Этот репозиторий не поддерживается. Используйте [Этот](https://github.com/Toveal/jsonschema-1c)

# Надстройка над крейтом **[jsonschema](https://crates.io/crates/jsonschema)** для 1С

## Свойства

> **:warning:** Вызывает исключение если схема не прошла валидацию

- Schema (Схема)
  - Хранит JSON схему. При установке значения свойства компилирует схему. Выдает исключение если схема не валидна
- Format (Формат)
  - Хранит формат результата метода Validate (проверить). Пример: "{path} - {instance} - {schema_path}".

## Методы

### Оба метода выдают исключение если

> **:warning:** Оба метода вызывают исключение если:
>
> - Не установлена схема
> - Передан не валидный JSON

- IsValid (Действителен)
  - Вернет булево по результату проверки. True - JSON прошел валидацию, false - не прошел
- Validate (Проверить)
  - Вернет булево и запишет результат проверки в аргумент функции. Если установлен формат, то массив ошибок по установленному формату. Иначе стандартный результат крейта **[JSONSchema](https://crates.io/crates/jsonschema)**

## Пример

```bsl
Функция ОбъектКомпоненты()

	// Установка, подключение, создание компоненты

	// Компиляция схемы
	ВК.Схема = СхемаJSON();
	Возврат ВК;

КонецФункции

Функция ПроверитьВалидностьJSON(JSON)

	// Установка, подключение, создание компоненты
	ВК = ОбъектКомпоненты();

	// Вернет Истина или Ложь
	Возврат ВК.Действителен(JSON);

КонецФункции

Функция ПроверитьJSONНаОшибкиБезФормата(JSON)

	// Установка, подключение, создание компоненты
	ВК = ОбъектКомпоненты();

	СтруктураРезультат = Новый Структура("ЕстьОшибка, Описание");

	// Запишет стандартный результат крейта JSONSchema в виде строки JSON
	БуферОшибок = "";

	СтруктураРезультат.ЕстьОшибка = ВК.Проверить(JSON, БуферОшибок);
	СтруктураРезультат.Описание = БуферОшибок;
	Возврат СтруктураРезультат;

КонецФункции

Функция ПроверитьJSONНаОшибкиСФорматом(JSON, Формат)

	// Установка, подключение, создание компоненты
	ВК = ОбъектКомпоненты();

	// Формат вывода результата
	ВК.Формат = Формат;

	СтруктураРезультат = Новый Структура("ЕстьОшибка, Описание");

	// Запишет массив JSON. Каждый элемент массива ошибка в формате установленном выше
	БуферОшибок = "";

	СтруктураРезультат.ЕстьОшибка = ВК.Проверить(JSON, БуферОшибок);
	СтруктураРезультат.Описание = БуферОшибок;
	Возврат СтруктураРезультат;

КонецФункции
```
