import xmlrpc.client

# Conectando-se ao servidor RPC
server = xmlrpc.client.ServerProxy("http://localhost:8000")

# Funções da calculadora
def menu():
    print("Escolha a operação:")
    print("1. Soma")
    print("2. Subtração")
    print("3. Multiplicação")
    print("4. Divisão")
    print("5. Sair")
    
def calcular():
    while True:
        menu()
        opcao = input("Digite a opção desejada (1-5): ")

        if opcao == '1':
            a = float(input("Digite o primeiro número: "))
            b = float(input("Digite o segundo número: "))
            resultado = server.soma(a, b)
            print(f"Resultado: {resultado}")
        elif opcao == '2':
            a = float(input("Digite o primeiro número: "))
            b = float(input("Digite o segundo número: "))
            resultado = server.subtrai(a, b)
            print(f"Resultado: {resultado}")
        elif opcao == '3':
            a = float(input("Digite o primeiro número: "))
            b = float(input("Digite o segundo número: "))
            resultado = server.multiplica(a, b)
            print(f"Resultado: {resultado}")
        elif opcao == '4':
            a = float(input("Digite o primeiro número: "))
            b = float(input("Digite o segundo número: "))
            try:
                resultado = server.divide(a, b)
                print(f"Resultado: {resultado}")
            except xmlrpc.client.Fault as e:
                print(f"Erro: {e.faultString}")
        elif opcao == '5':
            print("Saindo da calculadora...")
            break
        else:
            print("Opção inválida. Tente novamente.")

if __name__ == "__main__":
    calcular()

