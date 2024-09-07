fn main() {
    let arr = [1, 6, 4, 9, 7, 2];
    let arr_riordinato = merge_sort(&arr);

    println!("Array da riordinare {:?}", arr);
    println!("Array riordinato: {:?}", arr_riordinato)
}

fn merge_sort(arr:&[i32]) -> Vec<i32>{
    // Verifica della lunghezza del array
    if arr.len() <= 1{
        return arr.to_vec();
    }

    // divisione del array di input in due parti (sx, dx)
    let mid:usize = (arr.len()) / 2;
    let sx = merge_sort(&arr[..mid]);
    let dx = merge_sort(&arr[mid..]);

    // valore restituito
    merge(&sx, &dx)
}

fn merge(sx:&[i32], dx:&[i32]) -> Vec<i32>{
    let mut result:Vec<i32> = Vec::with_capacity(sx.len() + dx.len());

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