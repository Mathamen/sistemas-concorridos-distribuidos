from xmlrpc.server import SimpleXMLRPCServer

# Funções de operação da calculadora
def soma(a, b):
    return a + b

def subtrai(a, b):
    return a - b

def multiplica(a, b):
    return a * b

def divide(a, b):
    if b == 0:
        raise ValueError("Divisão por zero não permitida.")
    return a / b

# Criação do servidor RPC
server = SimpleXMLRPCServer(('localhost', 8000))
print("Servidor RPC iniciado em http://localhost:8000")

# Registrando as funções de operação
server.register_function(soma, 'soma')
server.register_function(subtrai, 'subtrai')
server.register_function(multiplica, 'multiplica')
server.register_function(divide, 'divide')

# Mantendo o servidor rodando
server.serve_forever()

