fn main() {
    let arr = [1.6, 6.4, 4.0, 9.5, 7.6, 2.2];
    let arr_riordinato = merge_sort(&arr);

    println!("Array da riordinare {:?}", arr);
    println!("Array riordinato: {:?}", arr_riordinato)
}

fn merge_sort(arr:&[f32]) -> Vec<f32>{
    // Verifica della lunghezza del array
    if arr.len() <= 1{
        return arr.to_vec();
    }

    // divisione del array di input in due parti (sx, dx)
    let mid:usize = (arr.len()) / 2;

    // calcolo di sx e dx in parallelo
    let (sx, dx): (Vec<f32>, Vec<f32>) =rayon::join(
        || merge_sort(&arr[..mid]),
        || merge_sort(&arr[mid..])
    );

    // valore restituito
    merge(&sx, &dx)
}

fn merge(sx:&[f32], dx:&[f32]) -> Vec<f32>{
    let mut result:Vec<f32> = Vec::with_capacity(sx.len() + dx.len());

    let mut index_i = 0;
    let mut index_j = 0;

    // Merging finché ci sono elementi in entrambe le parti
    while index_i < sx.len() && index_j < dx.len() {
        if sx[index_i] <= dx[index_j] {
            result.push(sx[index_i]);
            index_i += 1;
        } else {
            result.push(dx[index_j]);
            index_j += 1;
        }
    }

    // Aggiungi i restanti elementi di sx
    while index_i < sx.len() {
        result.push(sx[index_i]);
        index_i += 1;
    }

    // Aggiungi i restanti elementi di dx
    while index_j < dx.len() {
        result.push(dx[index_j]);
        index_j += 1;
    }

    result
}