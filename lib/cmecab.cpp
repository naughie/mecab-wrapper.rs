#include <mecab.h>

using namespace MeCab;

extern "C" const char *get_global_error() {
    return getLastError();
}

extern "C" void *new_model_argv(int argc, char **argv) {
    Model *model = createModel(argc, argv);
    void *void_model = (void *)model;
    return void_model;
}

extern "C" void *new_model_single(const char *arg) {
    Model *model = createModel(arg);
    void *void_model = (void *)model;
    return void_model;
}

extern "C" void delete_model(void *void_model) {
    Model *model = (Model *)void_model;
    delete model;
}

extern "C" void *dictionary_info(void *void_model) {
    Model *model = (Model *)void_model;
    const DictionaryInfo *info = model->dictionary_info();
    void *void_info = (void *)info;
    return void_info;
}

extern "C" const char *model_version(void *void_model) {
    Model *model = (Model *)void_model;
    return model->version();
}

extern "C" int transition_cost(void *void_model, unsigned short rattr, unsigned short lattr) {
    Model *model = (Model *)void_model;
    return model->transition_cost(rattr, lattr);
}

extern "C" bool swap_model(void *void_model, void *void_new_model) {
    Model *model = (Model *)void_model;
    Model *new_model = (Model *)void_new_model;

    return model->swap(new_model);
}

extern "C" void *model_lookup(void *void_model, const char *begin, const char *end, void *void_lattice) {
    Model *model = (Model *)void_model;
    Lattice *lattice = (Lattice *)void_lattice;

    Node *node = model->lookup(begin, end, lattice);
    void *void_node = (void *)node;
    return void_node;
}

extern "C" void *new_tagger(void *void_model) {
    Model *model = (Model *)void_model;
    Tagger *tagger = model->createTagger();
    void *void_tagger = (void *)tagger;
    return void_tagger;
}

extern "C" void delete_tagger(void *void_tagger) {
    Tagger *tagger = (Tagger *)void_tagger;
    delete tagger;
}

extern "C" const char *tagger_what(void *void_tagger) {
    Tagger *tagger = (Tagger *)void_tagger;
    return tagger->what();
}

extern "C" const char *tagger_version(void *void_tagger) {
    Tagger *tagger = (Tagger *)void_tagger;
    return tagger->version();
}

extern "C" void *new_lattice(void *void_model) {
    Model *model = (Model *)void_model;
    Lattice *lattice = model->createLattice();
    void *void_lattice = (void *)lattice;
    return void_lattice;
}

extern "C" void delete_lattice(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    delete lattice;
}

extern "C" const char *lattice_what(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->what();
}
extern "C" void set_lattice_what(void *void_lattice, const char *what) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_what(what);
}

extern "C" void set_sentence(void *void_lattice, const char *input, size_t len) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_sentence(input, len);
}

extern "C" bool parse(void *void_tagger, void *void_lattice) {
    Tagger *tagger = (Tagger *)void_tagger;
    Lattice *lattice = (Lattice *)void_lattice;

    return tagger->parse(lattice);
}

extern "C" const char *lattice_to_string(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    return lattice->toString();
}

extern "C" void *bos_node(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    const Node* bos = lattice->bos_node();
    void *void_node = (void *)bos;
    return void_node;
}

extern "C" void *eos_node(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    const Node* eos = lattice->eos_node();
    void *void_node = (void *)eos;
    return void_node;
}

extern "C" int get_request_type(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->request_type();
}

extern "C" void set_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_request_type(request_type);
}

extern "C" void add_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->add_request_type(request_type);
}

extern "C" void remove_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->remove_request_type(request_type);
}

extern "C" bool next_lattice(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->next();
}

extern "C" void *next_node(void *void_node) {
    const Node* node = (const Node*)void_node;
    return node->next;
}
