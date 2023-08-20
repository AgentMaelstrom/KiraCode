#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

using namespace std;

//===----------------------------------------------------------------------
// Lexer
//===----------------------------------------------------------------------

enum Token {
	t_identifier,
	t_modifier,
	t_def,
	t_type,
	t_separator,
	t_oper,
	t_num_literal,
	t_str_literal,
    t_other_literal
};

static string IdentifierStr;
static double NumVal;

//===----------------------------------------------------------------------
//Abstract Syntax Tree (aka Parse Tree)
//===----------------------------------------------------------------------

namespace {

class ExpressionAST {
	public:
		virtual ~ExpressionAST() = default;
};

class NumExprAST : public ExpressionAST {
	double Value;

	public:
	    NumExprAST(double Value) : Value(Value) {}
};

class VarExprAST : public ExpressionAST {
	string Name;

	public:
	    VarExprAST(const string &Name) : Name(Name) {}
};

class BinExprAST : public ExpressionAST {
	char Op;
	unique_ptr<ExpressionAST> LHS, RHS;

	public:
	    BinExprAST(char Op, unique_ptr<ExpressionAST> LHS, 
				   unique_ptr<ExpressionAST> RHS) : Op(Op), 
				   LHS(move(LHS)), RHS(move(RHS)) {}
};

class CallExprAST : public ExpressionAST {
	// tbc
};

}

//===----------------------------------------------------------------------
// Parser
//===----------------------------------------------------------------------

int getNextToken() {
	throw ("Method 'getNextToken()', line 73 not implemented");

	return 0;
}

int parser() {
	getNextToken();
	vector<unique_ptr<ExpressionAST>> Args;
	if (CurrentToken != ')') {
		while (true) {
			if (auto Arg = ParseExpression())
				Args.push_back(move(Arg));
			else
				return nullptr
		}
	}
}


int main() {
	
	return 0;
}
