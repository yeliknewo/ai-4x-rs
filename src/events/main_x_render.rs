use graphics::NGEncoder;

pub enum MainFromRender<ID>
    where ID: Send
{
    Encoder(NGEncoder, ID),
    Exited,
}

pub enum MainToRender<ID>
    where ID: Send
{
    Encoder(NGEncoder, ID),
    Exit,
}
