use std::ops::Sub;
use std::io::BufRead;

/* 
pub fn elementwise_subtraction<N, IA, IB, F>(a: IA, b: IB) -> F 
where 
    N: Sub,
    IA: IntoIterator<Item = N>,
    IB: IntoIterator<Item = N>,
    F: FromIterator<N> + FromIterator<<N as Sub>::Output>,
*/
pub fn elementwise_subtraction(a: Vec<i64>, b: Vec<i64>) -> Vec<i64>
    {
        a.into_iter().zip(b).map(|(a,b)| a.abs_diff(b) as i64).collect::<Vec<i64>>()
    }



    fn read_file(path: impl Into<String>) -> Option<(Vec<i64>, Vec<i64>)>{
        fn __read_file(path: &str) -> Option<(Vec<i64>, Vec<i64>)>{
            let f = std::fs::File::open(path).ok()?;
            let reader = std::io::BufReader::new(f);
    
            let mut list1: Vec<i64>= vec![];
            let mut list2: Vec<i64> = vec![];
            reader.lines().for_each(|line|{
                if let Ok(s) = line {
                    let sp = s.split_ascii_whitespace().collect::<Vec<_>>();
                    if let Ok(i1) = sp[0].parse::<i64>(){
                        list1.push(i1);
                    }
                    if let Ok(i2) = sp[1].parse::<i64>(){
                        list2.push(i2);
                    }
                }
            });
            list1.sort_unstable();
            list2.sort_unstable();
            Some((list1, list2))
        }
        __read_file(path.into().as_str())
    }


fn main() {

    let r = read_file("C:/Users/bandi/projects/rustcargo/advent1/input1");    
    if r.is_none(){
        println!("file error");
        return;
    }
    let (list1, list2) = r.unwrap();

    let list3: Vec<_> = elementwise_subtraction(list1, list2);
    let res: i64 = list3.iter().sum();
    println!("{}", res)

}
