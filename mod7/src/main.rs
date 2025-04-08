// In Class Assignment 
//Create a struct Student (major)
// Higher order functions update majors 
//FIrst oder functions, assign_major(student,major_declared)
//create a vector of students1,2,3 and update all studnets major

struct Student {
    major: String,
}

// First-order function to assign a major to a student
fn assign_major(student: &mut Student, major: String) {
    student.major = major;
}

// Higher-order function that applies a behavior to update each student's major
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) -> Vec<Student> {
    let majors = vec![
        "Computer Science".to_string(),
        "Computer Enigneering".to_string(),
        "Physics".to_string(),
    ];

    for (student, major) in collection.iter_mut().zip(majors.into_iter()) {
        behavior(student, major);
    }

    collection
}

fn main() {
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    let updated_students = update_majors(students, assign_major);

    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}
