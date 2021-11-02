impl Rectangle
{
    fn width(&self)->bool
    {
        self.width>0
    }
}
fn main()
{
    let rect1=Rectangle{
    width:30,
    height:50
};

if rect.width()
{
    println!("width is {}",rect.width);
}
}