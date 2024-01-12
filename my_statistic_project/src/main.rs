use serde::Deserialize;

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
struct WorkerSalary {
    workers: Vec<SalaryRecord>,
    regs_to_read: i32,
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

    fn show_list( &self ) {
        let linea = format!( 
            "| {:>9} | {:>14}   | {:>15} | {:>14} | {:>10} | {:>14}  | {:>14} | {:>18} | {:>12} | {:>16} |   {:>10} | ", 
            self.work_year , self.work_year, self.employment_type, self.job_title, self.salary, self.salary_currency, self.salary_in_usd, self.employee_residence, self.remote_ratio, self.company_location, self.company_size );
        println!( "{}", linea );
    }

    fn header_list( ) {
        let linea = format!( 
            "| {:>9} | {:>14} | {:>15} | {:>14} | {:>10} | {:>14} | {:>14} | {:>18} | {:>12} | {:>16} | {:>10} | ", 
            "Work Year", "Experience level", "Employment type", "Job title",  "Salary", "Salary Currency", "Salary in USD", "Employee residence","Remote ratio", "Company location", "Company size");
        println!( "{}", linea );
        let sub_line = format!("{:-<184}", " ");
        println!( "{}", sub_line );
    }

}


#[allow( dead_code )]
impl WorkerSalary {
    fn new( ) -> WorkerSalary {
        return WorkerSalary{ workers: Vec::new(), regs_to_read: 0 };
    }

    fn show_workers( &self ) {
        let mut contador: i32 = 0;

        for record in self.workers.iter() {
            if contador < self.regs_to_read {
                println!( "{} {} {} {} {} {} {} {} {} {} {} ",
                record.work_year,
                record.experience_level,
                record.employment_type,
                record.job_title,
                record.salary,
                record.salary_currency,
                record.salary_in_usd,
                record.employee_residence,
                record.remote_ratio,
                record.company_location,
                record.company_size);
                contador += 1;
            }
        }

    }

    #[allow(unused_variables)]
    fn read_data( fpath : String, nregs: Option<i32> ) -> Result< WorkerSalary, Box<dyn std::error::Error> >  {
        let ruta_file : String = String::from( fpath ) ; 
        let fopen = std::fs::File::open( ruta_file )?;
        let mut rdr = csv::Reader::from_reader( fopen );
        
        let mut wsalary : WorkerSalary = WorkerSalary::new();

        for record in rdr.deserialize() {
            let record : SalaryRecord = record?;
            wsalary.workers.push( record );    
        }

        let regs = match nregs {
            Some( regs ) => wsalary.regs_to_read = regs as i32,
            None => wsalary.regs_to_read = wsalary.workers.len() as i32,
        };

        Ok( wsalary )
    }

    fn read_salaries( &self ) -> Vec::<f64> {
        let mut salaries : Vec<f64> = Vec::new();
        for record in self.workers.iter() {
            salaries.push( record.salary );
        }
        return salaries;
    }
}

fn calculate_median( data: Vec<f64> ) -> f64 {
    let sum : f64 = data.iter().sum();
    sum / data.len() as f64
}

#[ allow( unused_assignments )]
fn main() {
    
    let precision = 2;
    let path_to_file = "csvs/salaries.csv".to_string();
    let show_regs : i32 = 4;

    match WorkerSalary::read_data( path_to_file, Some(show_regs) ) {
        Ok( wsalary) => { 
            //wsalary.show_workers();
            println!( "Median of salaries: {:.1$}", calculate_median( wsalary.read_salaries() ), precision );
        }
        Err( _error ) => {println!( "Error al cargar datos en la clase"); }
    }

} 