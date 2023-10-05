use native_1c::component::{IComponentBase, IComponentInit};
use native_1c::native_macro::native_object;
use native_1c::types::Variant;

#[native_object]
#[repr(C)]
pub struct JsonSchema {
    schema_str: String,
    schema: Option<jsonschema::JSONSchema>,
    output_format: Option<String>,
}

impl IComponentBase for JsonSchema {
    fn init(&mut self) -> bool {
        true
    }

    fn get_info(&self) -> i32 {
        2000
    }

    fn done(&mut self) {}

    fn get_n_props(&self) -> i32 {
        2
    }

    fn find_prop(&self, prop_name: &str) -> i32 {
        match prop_name {
            "Schema" | "Схема" => 0,
            "Format" | "Формат" => 1,
            _ => -1,
        }
    }

    fn get_prop_name(&self, prop_num: i32, prop_alias: i32) -> &str {
        match prop_num {
            0 => {
                if prop_alias == 0 {
                    "Schema"
                } else {
                    "Схема"
                }
            }
            1 => {
                if prop_alias == 0 {
                    "Format"
                } else {
                    "Формат"
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_prop_val(&self, prop_num: i32, var_prop_val: &mut Variant) -> bool {
        match prop_num {
            0 => *var_prop_val = Variant::utf8_string(self, &self.schema_str),
            1 => {
                let val = match &self.output_format {
                    Some(value) => value.clone(),
                    _ => "".to_string(),
                };
                *var_prop_val = Variant::utf16_string(self, &val);
            }
            _ => return false,
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i32, var_prop_val: &Variant) -> bool {
        match prop_num {
            0 => match var_prop_val.as_string() {
                Some(val) => match self.set_schema(val) {
                    Err(_) => {
                        self.raise_an_exception("Схема не прошла валидацию");
                        self.schema_str = "".to_string();
                    }
                    Ok(_) => {}
                },
                _ => return false,
            },
            1 => match var_prop_val.as_string() {
                Some(value) => self.output_format = Some(value),
                None => self.output_format = None,
            },
            _ => return false,
        }
        true
    }
    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, _prop_num: i32) -> bool {
        true
    }

    fn get_n_methods(&self) -> i32 {
        2
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            "IsValid" | "Действителен" => 0,
            "Validate" | "Проверить" => 1,
            _ => -1,
        }
    }
    fn get_method_name(&self, method_num: i32, method_alias: i32) -> &str {
        match method_num {
            0 => {
                if method_alias == 0 {
                    "IsValid"
                } else {
                    "Действителен"
                }
            }
            1 => {
                if method_alias == 0 {
                    "Validate"
                } else {
                    "Проверить"
                }
            }
            _ => unreachable!(),
        }
    }
    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            0 => 1,
            1 => 2,
            _ => 0,
        }
    }

    #[allow(unused_variables, unreachable_code)]
    fn get_param_def_value(
        &self,
        method_num: i32,
        param_num: i32,
        var_param_def_value: &mut Variant,
    ) -> bool {
        match method_num {
            _ => return false,
        }
        true
    }
    fn has_ret_val(&self, _method_num: i32) -> bool {
        true
    }

    fn call_as_proc(&mut self, _method_num: i32, _params: Option<&mut [Variant]>) -> bool {
        false
    }

    fn call_as_func(
        &mut self,
        method_num: i32,
        ret_vals: &mut Variant,
        params: Option<&mut [Variant]>,
    ) -> bool {
        match method_num {
            0 | 1 => {
                if self.schema.is_none() {
                    self.raise_an_exception("Схема не установлена");
                    return false;
                }

                let params_mut = params.unwrap();
                let json = params_mut.get(0).unwrap().as_string().unwrap();

                if method_num == 0 {
                    *ret_vals = Variant::from(self.is_valid(json));
                } else {
                    let mut buffer = String::new();
                    *ret_vals = Variant::from(self.validate(json, &mut buffer));
                    params_mut[1] = Variant::utf16_string(self, &buffer);
                }
            }
            _ => return false,
        }
        true
    }

    fn set_locale(&mut self, _loc: &str) {}
}

impl JsonSchema {
    fn is_valid(&self, json: String) -> bool {
        match self.schema.as_ref() {
            Some(schema) => match serde_json::from_str::<serde_json::Value>(&json) {
                Ok(json_value) => schema.is_valid(&json_value),
                Err(_) => {
                    self.raise_an_exception("Неверный формат JSON");
                    false
                }
            },
            None => {
                self.raise_an_exception("Схема не установлена");
                false
            }
        }
    }

    fn validate(&self, json: String, result: &mut String) -> bool {
        let schema = match self.schema.as_ref() {
            Some(s) => s,
            None => {
                self.raise_an_exception("Схема не установлена");
                return false;
            }
        };

        let json_value = match serde_json::from_str::<serde_json::Value>(&json) {
            Ok(json) => json,
            Err(_) => {
                self.raise_an_exception("Не удалось преобразовать в JSON");
                return false;
            }
        };

        let output_json: Result<String, serde_json::Error> =
            if let Some(ref format) = self.output_format {
                let errors_vec: Vec<String> = match schema.validate(&json_value) {
                    Ok(_) => vec![],
                    Err(errors) => errors
                        .map(|err| {
                            format
                                .replace("{path}", &err.instance_path.to_string())
                                .replace("{instance}", &err.instance.to_string())
                                .replace("{schema_path}", &err.schema_path.to_string())
                        })
                        .collect(),
                };
                serde_json::to_string(&errors_vec)
            } else {
                let output = schema.apply(&json_value).basic();
                serde_json::to_string(&output)
            };

        match output_json {
            Ok(out) => {
                *result = out;
                true
            }
            Err(_) => {
                self.raise_an_exception("Не удалось преобразовать результат проверки в JSON");
                false
            }
        }
    }

    fn raise_an_exception(&self, text: &str) {
        self.connector()
            .add_error(1006, "JsonSchema", text, 1, self.mem_manager());
    }

    fn set_schema(&mut self, text: String) -> Result<(), ()> {
        let json_inp: serde_json::Value = match serde_json::from_str(&text) {
            Ok(json) => json,
            _ => return Err(()),
        };

        let schema = match jsonschema::JSONSchema::compile(&json_inp) {
            Ok(schema) => schema,
            Err(_) => return Err(()),
        };

        self.schema = Some(schema);
        self.schema_str = text;
        return Ok(());
    }
}
