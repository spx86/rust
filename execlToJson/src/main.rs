use calamine::{open_workbook, Reader, Xlsx};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Clone, Debug)]
struct JsonRoot {
    categoryModelName: String,
    creator: String,
    categories: Vec<JsonCategory>,
}

#[derive(Serialize, Clone, Debug)]
struct JsonCategory {
    level: String,
    categoryName: String,
    categoryDescription: String,
    subcategories: Vec<JsonSubCategory>,
}

#[derive(Serialize, Clone, Debug)]
struct JsonSubCategory {
    level: String,
    categoryName: String,
    categoryDescription: String,
    subcategories: Vec<JsonLevel3>,
}

#[derive(Serialize, Clone,Debug)]
struct JsonLevel3 {
    level: String,
    categoryName: String,
    categoryDescription: String,
    dateTypes: Vec<DateType>,
}

#[derive(Serialize, Clone, Debug)]
struct DateType {
    dataTypeName: String,
    attributeIdentifier: String,
    dataLevel: u8,
    dataFeature: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Excel 文件路径
    let file_path = "data.xlsx";

    // 打开 Excel 文件
    let mut workbook: Xlsx<_> = open_workbook(file_path)?;
    let sheet_name = "数据分类分级标准";

    // 获取工作表
    let sheet = workbook.worksheet_range(sheet_name).ok_or("工作表不存在")??;

    let mut categories = Vec::new();
    let mut current_category: Option<String> = None;
    let mut current_subcategory: Option<String> = None;
    let mut level3_items = Vec::new();

    for row in sheet.rows().skip(1) {
        let category = row[0].to_string();
        let subcategory = row[1].to_string();
        let description = row[2].to_string();
        let contents = row[3].to_string();
        let tags = row[4].to_string();
        let level: u8 = row[5].to_string().parse().unwrap_or(0);
        let importance = row[6].to_string();

        if let Some(current_cat) = &current_category {
            if *current_cat != category && !category.is_empty() {
                categories.push(JsonCategory {
                    level: "1".to_string(),
                    categoryName: current_cat.clone(),
                    categoryDescription: "".to_string(),
                    subcategories: vec![JsonSubCategory {
                        level: "2".to_string(),
                        categoryName: current_subcategory.clone().unwrap_or_default(),
                        categoryDescription: description.clone(),
                        subcategories: level3_items.clone(),
                    }],
                });
                current_subcategory = None;
                level3_items.clear();
            }
        }

        if category != current_category.clone().unwrap_or_default() {
            current_category = Some(category.clone());
        }

        if subcategory != current_subcategory.clone().unwrap_or_default() {
            current_subcategory = Some(subcategory.clone());
        }

        // 处理内容，根据“、”分割
        let date_types = contents
            .split('、')
            .map(|content| DateType {
                dataTypeName: content.trim().to_string(),
                attributeIdentifier: tags.to_string(), // 可根据需要填充
                dataLevel: level,
                dataFeature:  content.trim().to_string(),
            })
            .collect();

        level3_items.push(JsonLevel3 {
            level: "3".to_string(),
            categoryName: subcategory.clone(),
            categoryDescription: description.clone(),
            dateTypes: date_types,
        });
    }

    if let Some(current_cat) = &current_category {
        categories.push(JsonCategory {
            level: "1".to_string(),
            categoryName: current_cat.clone(),
            categoryDescription: "".to_string(),
            subcategories: vec![JsonSubCategory {
                level: "2".to_string(),
                categoryName: current_subcategory.clone().unwrap_or_default(),
                categoryDescription: "".to_string(),
                subcategories: level3_items.clone(),
            }],
        });
    }

    let root = JsonRoot {
        categoryModelName: "医保局医疗信息分级分类模板".to_string(),
        creator: "spengxu".to_string(),
        categories,
    };

    // 将数据转换为 JSON 格式
    let json_data = serde_json::to_string_pretty(&root)?;

    // 输出 JSON 数据到文件
    let output_path = "output.json";
    let mut output_file = File::create(output_path)?;
    output_file.write_all(json_data.as_bytes())?;

    println!("JSON 数据已生成并保存到 {}", output_path);

    Ok(())
}
