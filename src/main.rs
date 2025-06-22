use serde::{Serialize, Deserialize};
use std::io::{self, Write};

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, Serialize, Deserialize)]
struct Student{
 name: String,
 total_marks: f32,
 num_subjects: u32,
 average:f32,
 grade: char,   
}

fn calculate_avg(total: f32, subjects: u32)-> f32{
    total / subjects as f32
}

fn assign_grade(avg: f32)-> char{
    match avg {
        x if x >= 90.0 => 'A',
        x if x >= 75.0 => 'B',
        x if x >= 60.0 => 'C',
        _ => 'D',
    }   
}


fn generate_pdf(student: &Student){
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let lines = vec![
        format!("📄 Report Card for {}", student.name),
        format!("Total Marks: {}", student.total_marks),
        format!("Subjects: {}", student.num_subjects),
        format!("Average: {}", student.average),
        format!("Grade: {}", student.grade),
    ];

    let mut y = Mm(270.0);
    for line in lines{
        current_layer.use_text(line, 16.0, Mm(20.0), y, &font);
        y -= Mm(12.0);
    }

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();
}

fn main(){
 println!("📝 Student Report Card Generator");

    print!("Enter student name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter total marks: ");
    io::stdout().flush().unwrap();
    let mut marks_input = String::new();
    io::stdin().read_line(&mut marks_input).unwrap();
    let total_marks: f32 = marks_input.trim().parse().expect("Please enter valid marks");

    print!("Enter number of subjects: ");
    io::stdout().flush().unwrap();
    let mut sub_input = String::new();
    io::stdin().read_line(&mut sub_input).unwrap();
    let num_subjects: u32 = sub_input.trim().parse().expect("Please enter valid number");

    let average = calculate_avg(total_marks, num_subjects);
    let grade = assign_grade(average);

    let student = Student {
        name: name.trim().to_string(),
        total_marks,
        num_subjects,
        average,
        grade,
    };

    println!("\n✅ Report Card Generated:\n{:#?}", student);

    generate_pdf(&student);
    println!("📄 PDF saved as 'report_card.pdf'");
}