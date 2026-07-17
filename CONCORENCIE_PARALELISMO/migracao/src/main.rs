use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Cliente {
    id: u32,
    nome: String,
}

struct Produto {
    id: u32,
    nome: String,
}

struct Pedido {
    cliente_id: u32,
    produto_ids: Vec<u32>,
}

fn main() {
    let (tx_clientes, rx_clientes) = mpsc::channel();
    let (tx_produtos, rx_produtos) = mpsc::channel();

    // Thread para criar clientes
    let cliente_thread = thread::spawn(move || {
        // trazendo do banco de dados simulação
        let clientes = vec![
            Cliente { id: 1, nome: "Cliente 1".into() },
            Cliente { id: 2, nome: "Cliente 2".into() },
        ];

        for cliente in clientes {
            println!("Enviando cliente {} ...", cliente.nome);
            tx_clientes.send(cliente).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Thread para criar produtos
    let produto_thread = thread::spawn(move || {
        // Simulação de ler produtos do banco de dados
        let produtos = vec![
            Produto { id: 1, nome: "Produto 1".into() },
            Produto { id: 2, nome: "Produto 2".into() },
        ];

        for produto in produtos {
            println!("Enviando produto {} ...", produto.nome);
            tx_produtos.send(produto).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Espera as threads terminarem para garantir que todos os dados foram enviados
    //=== certo
    cliente_thread.join().unwrap();
    produto_thread.join().unwrap();

    //=== errado
    // thread::sleep(Duration::from_millis(1000));


    // Coleta clientes e produtos após as threads terem terminado
    let clientes: Vec<Cliente> = rx_clientes.try_iter().collect();
    let produtos: Vec<Produto> = rx_produtos.try_iter().collect();

    // Criação de pedido
    if !clientes.is_empty() && !produtos.is_empty() {
        let pedido = Pedido {
            cliente_id: clientes[0].id,
            produto_ids: produtos.iter().map(|p| p.id).collect(),
        };

        println!("Pedido criado para o cliente ID: {} com os produtos IDs: {:?}", pedido.cliente_id, pedido.produto_ids);
    } else {
        println!("Não há clientes ou produtos suficientes para criar um pedido.");
    }
}
