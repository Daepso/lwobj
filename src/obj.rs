use std::io::BufRead;
use std::io;
use std::str::SplitWhitespace;
use std::str::FromStr;

#[derive(Debug)]
pub enum LoadingError {
    InvalidLine(usize),
    WrongNumberOfArguments(usize),
    Parse(usize),
    Io(io::Error),
}

/// A struct containing all data store by wavefront
pub struct ObjData {
    vertices : Vec<(f32,f32,f32,f32)>,
    normals : Vec<(f32,f32,f32)>,
    texcoords : Vec<(f32,f32,f32)>,
    faces : Vec<Vec<(usize,Option<usize>,Option<usize>)>>
}

impl From<io::Error> for LoadingError {
    fn from(err : io::Error) -> LoadingError {
        LoadingError::Io(err)
    }
}

fn parse<T : FromStr>(it : SplitWhitespace, nb : usize) -> Result<Vec<T>, LoadingError> {
    let mut vec : Vec<T> = Vec::new();
    for s in it {
        let val = match s.parse::<T>() {
            Ok(v) => v,
            Err(_) => return Err(LoadingError::Parse(nb)),
        };
        vec.push(val);
    }
    return Ok(vec);
}

impl ObjData {

    /// Constructs a new empty `ObjData`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwobj::ObjData;
    ///
    /// let data = ObjData::new();
    /// ```
    pub fn new() -> ObjData {
        ObjData {
            vertices : Vec::new(),
            normals : Vec::new(),
            texcoords : Vec::new(),
            faces : Vec::new(),
        }
    }


    /// Load an `ObjData` from a `BufReader`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use lwobj::ObjData;
    ///
    /// let f = File::open("cube.obj").unwrap();
    /// let mut input = BufReader::new(f);
    /// let data = ObjData::load(&mut input).ok().unwrap();
    /// ```
    pub fn load<R : io::Read>(input : &mut io::BufReader<R>) -> Result<ObjData,LoadingError> {
        let mut data = ObjData::new();
        let mut buf = String::new();
        let mut nb : usize = 0;
        while try!(input.read_line(&mut buf)) > 0 {
            // Skip comment
            if buf.chars().next().unwrap() != '#' {
                let mut iter = buf.split_whitespace();
                match iter.next() {
                    Some("v") => {
                        let args = try!(parse::<f32>(iter,nb));
                        if args.len() == 4 {
                            data.vertices.push((args[0],args[1],args[2],args[3]));
                        } else if args.len() == 3 {
                            data.vertices.push((args[0],args[1],args[2],1.0));
                        } else {
                            return Err(LoadingError::WrongNumberOfArguments(nb));
                        }
                    },
                    Some("vn") => {
                        let args = try!(parse::<f32>(iter,nb));
                        if args.len() == 3 {
                            data.normals.push((args[0],args[1],args[2]));
                        } else {
                            return Err(LoadingError::WrongNumberOfArguments(nb));
                        }
                    },
                    Some("vt") => {
                        let args = try!(parse::<f32>(iter,nb));
                        if args.len() == 3 {
                            data.texcoords.push((args[0],args[1],args[2]));
                        } else if args.len() == 2 {
                            data.texcoords.push((args[0],args[1],0.));
                        } else {
                            return Err(LoadingError::WrongNumberOfArguments(nb));
                        }
                    },
                    Some("s") => {
                        // Not supported
                    },
                    Some("f") => {
                        let mut vec : Vec<(usize,Option<usize>,Option<usize>)> = Vec::new();
                        for arg in iter {
                            let index : Vec<_> = arg.split('/').collect();
                            if index.len() != 3 {
                                return Err(LoadingError::WrongNumberOfArguments(nb));
                            }
                            let v = match index[0].parse::<usize>() {
                                Ok(val) => val,
                                Err(_) => return Err(LoadingError::Parse(nb)),
                            };
                            let vt = index[1].parse::<usize>().ok();
                            let vn = index[2].parse::<usize>().ok();
                            vec.push((v,vt,vn));
                        }
                        data.faces.push(vec);
                    },
                    Some("o") => {
                        // Not supported
                    },
                    _ => return Err(LoadingError::InvalidLine(nb)),
                }
            }
            nb += 1;
            buf.clear();
        }
        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use obj::*;

    #[test]
    fn load() {
        let mut expected = ObjData::new();
        expected.vertices = vec![(1.,-1.,-1.,1.),
        (1.,-1.,1.,1.),
        (-1.,-1.,1.,1.),
        (-1.,-1.,-1.,1.),
        (1.,1.,-1.,1.),
        (1.,1.,1.,1.),
        (-1.,1.,1.,1.),
        (-1.,1.,-1.,1.)];
        expected.normals = vec![(0.,-1.,0.),
        (0.,1.,0.),
        (1.,0.,0.),
        (0.,0.,1.),
        (-1.,0.,0.),
        (0.,0.,-1.)];
        expected.faces = vec![ vec![(2,None,Some(1)), (4,None,Some(1)), (1,None,Some(1))],
        vec![(8,None,Some(2)), (6,None,Some(2)), (5,None,Some(2))],
        vec![(5,None,Some(3)), (2,None,Some(3)), (1,None,Some(3))],
        vec![(6,None,Some(4)), (3,None,Some(4)), (2,None,Some(4))],
        vec![(3,None,Some(5)), (8,None,Some(5)), (4,None,Some(5))],
        vec![(1,None,Some(6)), (8,None,Some(6)), (5,None,Some(6))],
        vec![(2,None,Some(1)), (3,None,Some(1)), (4,None,Some(1))],
        vec![(8,None,Some(2)), (7,None,Some(2)), (6,None,Some(2))],
        vec![(5,None,Some(3)), (6,None,Some(3)), (2,None,Some(3))],
        vec![(6,None,Some(4)), (7,None,Some(4)), (3,None,Some(4))],
        vec![(3,None,Some(5)), (7,None,Some(5)), (8,None,Some(5))],
        vec![(1,None,Some(6)), (4,None,Some(6)), (8,None,Some(6))],
        ];
        let f = File::open("cube.obj").unwrap();
        let mut input = BufReader::new(f);
        let data = ObjData::load(&mut input).ok().unwrap();
        assert_eq!(expected.vertices,data.vertices);
        assert_eq!(expected.normals,data.normals);
        assert_eq!(expected.texcoords,data.texcoords);
        assert_eq!(expected.faces,data.faces);
    }
}