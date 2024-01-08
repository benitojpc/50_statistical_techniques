use serde::Deserialize;
use reqwest::blocking::get;
use std::io::Read;

use csv::ReaderBuilder;
use std::error::Error;

#[derive( Debug, Deserialize )]
#[allow( dead_code )]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
} 

#[allow( dead_code )]
impl SalaryRecord {

    fn new( ) -> SalaryRecord {
        return SalaryRecord {
            work_year: 0,
            experience_level: String::from(""),
            employment_type: String::from(""),
            job_title: String::from(""),
            salary: 0.0,
            salary_currency: String::from(""),
            salary_in_usd: 0.0,
            employee_residence: String::from(""),
            remote_ratio: 0.0,
            company_location: String::from(""),
            company_size: String::from(""),
        };
    }

    fn show_card( &self ) {
        println!( "{:>19}: {:>11}", "Work Year".to_string(), self.work_year );
        println!( "{:>19}: {:>11}", "Experience level".to_string(), self.work_year );
        println!( "{:>19}: {:>11}", "Employment type", self.employment_type );
        println!( "{:>19}: {:>11}", "Job title", self.job_title );
        println!( "{:>19}: {:>11}", "Salary", self.salary );
        println!( "{:>19}: {:>11}", "Salary Currency", self.salary_currency );
        println!( "{:>19}: {:>11}", "Salary in USD", self.salary_in_usd );
        println!( "{:>19}: {:>11}", "Employee residence", self.employee_residence );
        println!( "{:>19}: {:>11}", "Remote ratio", self.remote_ratio );
        println!( "{:>19}: {:>11}", "Company location", self.company_location );
        println!( "{:>19}: {:>11}", "Company size", self.company_size );
    }

    fn header_list( ) {
        let linea = format!( 
            "| {:>9} | {:>14} | {:>15} | {:>14} | {:>10} | {:>14} | {:>14} | {:>18} | {:>12} | {:>16} | {:>10} | ", 
            "Work Year", "Experience level", "Employment type", "Job title",  "Salary", "Salary Currency", "Salary in USD", "Employee residence","Remote ratio", "Company location", "Company size");
        println!( "{}", linea );
        let sub_line = format!("{:-<184}", " ");
        println!( "{}", sub_line );
    }

    fn show_list( &self ) {
        let linea = format!( 
            "| {:>9} | {:>14}   | {:>15} | {:>14} | {:>10} | {:>14}  | {:>14} | {:>18} | {:>12} | {:>16} |   {:>10} | ", 
            self.work_year , self.work_year, self.employment_type, self.job_title, self.salary, self.salary_currency, self.salary_in_usd, self.employee_residence, self.remote_ratio, self.company_location, self.company_size );
        println!( "{}", linea );
    }

}

fn fetch_dataset( url: &str ) -> Result<String, Box<dyn Error>> {
    let mut response = get(url)?;
    let mut content = String::new();
    response.read_to_string( &mut content )?;
    Ok( content )
}

fn load_dataset( csv_data: &str) -> Result<Vec<SalaryRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader( csv_data.as_bytes() );
    let mut records = Vec::new();
  
    for result in reader.deserialize().skip( 1 ) {   // skip row for column's name
        let record: SalaryRecord = result?;
        records.push( record );
    }
    Ok( records )
}

fn show_dataset ( regs : Vec<SalaryRecord>, nregs: Option<usize>, lista: bool ) {
    let num_regs;
    match nregs {
            Some( nregs ) => {num_regs = nregs;},
            None  => {num_regs = regs.len();}
    }
    println!( "\n" );
    
    let mut primero : bool = true;

    if lista == true {
        SalaryRecord::header_list();
    }
    for (i, reg)  in regs.iter().enumerate() {
        if i < num_regs {
            let record = reg;
            if lista == false {
                if primero == true {
                    primero = false; 
                } else {
                    println!("\n ===============================\n" ); 
                }
                record.show_card();
            } else {
                //println!( "{:?}\n", reg );
                record.show_list();
            }
        }
    }
}

#[ allow( unused_assignments )]
fn main() {
    let url = "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv"; 
    //let url = "csvs/salaries.csv";

    let show_regs : usize = 4;
    
    match fetch_dataset( url ) {  
        Ok( csv_data ) => {  
            let ds = load_dataset( &csv_data );
            
            match ds {  
                Ok( dataset ) => {  
                    println!( "Loaded {} records", dataset.len() );  
                    show_dataset( dataset, Some(show_regs), true);
                } 
                Err( error ) => {  
                    eprintln!( "Error loading dataset: {}", error );  
                } 
            } 
        } 
        Err( error) => {   
            eprintln!( "Error fetching dataset: {}", error );  
        } 
    } 
  
} 